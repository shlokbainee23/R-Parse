//! Process management and IPC communication with the GNU R child process.

use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::{BufReader, BufWriter, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crate::config::RuntimeConfig;
use crate::error::RuntimeError;
use crate::json::JsonValue;
use crate::protocol::{read_message, write_message, RpcRequest, RpcResponse};

/// The embedded R worker script source code.
pub const R_WORKER_SCRIPT: &str = include_str!("worker.R");

static TEMP_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Backend abstraction for communicating with a runtime engine (real GNU R or mock).
pub trait RuntimeBackend: Send {
    /// Dispatches a JSON-RPC request and awaits the structured response with timeout.
    fn send_request(
        &mut self,
        method: &str,
        params: JsonValue,
        timeout_ms: u64,
    ) -> Result<RpcResponse, RuntimeError>;

    /// Returns true if the backend process or connection is active.
    fn is_alive(&self) -> bool;

    /// Forcefully terminates or cleans up the backend.
    fn shutdown(&mut self);
}

/// An active GNU R child process communicating over stdio.
pub struct RProcess {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    resp_receiver: Receiver<Result<RpcResponse, RuntimeError>>,
    temp_script_path: Option<PathBuf>,
    next_id: u64,
    is_alive: bool,
}

impl RProcess {
    /// Spawns a new GNU R process running the embedded worker script.
    pub fn spawn(executable: &Path, config: &RuntimeConfig) -> Result<Self, RuntimeError> {
        // Write the embedded worker script to a unique temporary file
        let unique_id = TEMP_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid = std::process::id();
        let temp_script = env::temp_dir().join(format!("rparse_worker_{}_{}.R", pid, unique_id));

        fs::write(&temp_script, R_WORKER_SCRIPT).map_err(|e| RuntimeError::ProcessSpawnFailed {
            path: executable.to_path_buf(),
            message: format!("Failed to write temporary worker script: {}", e),
        })?;

        let mut cmd = Command::new(executable);
        cmd.arg("--vanilla")
            .arg(&temp_script)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        if let Some(work_dir) = &config.work_dir {
            cmd.current_dir(work_dir);
        }

        for (k, v) in &config.env_vars {
            cmd.env(k, v);
        }

        let mut child = cmd.spawn().map_err(|e| {
            let _ = fs::remove_file(&temp_script);
            RuntimeError::ProcessSpawnFailed {
                path: executable.to_path_buf(),
                message: e.to_string(),
            }
        })?;

        let child_stdin = child.stdin.take().ok_or_else(|| {
            let _ = fs::remove_file(&temp_script);
            RuntimeError::IoError("Failed to open child process stdin".to_string())
        })?;

        let child_stdout = child.stdout.take().ok_or_else(|| {
            let _ = fs::remove_file(&temp_script);
            RuntimeError::IoError("Failed to open child process stdout".to_string())
        })?;

        let (tx, rx) = mpsc::channel();

        // Spawn a reader thread to handle incoming responses asynchronously
        thread::spawn(move || {
            let mut reader = BufReader::new(child_stdout);
            loop {
                match read_message(&mut reader) {
                    Ok(resp) => {
                        if tx.send(Ok(resp)).is_err() {
                            break;
                        }
                    }
                    Err(err) => {
                        let _ = tx.send(Err(err));
                        break;
                    }
                }
            }
        });

        let mut process = Self {
            child,
            stdin: BufWriter::new(child_stdin),
            resp_receiver: rx,
            temp_script_path: Some(temp_script),
            next_id: 1,
            is_alive: true,
        };

        // Quick handshake check
        let ping_res = process.send_request("ping", JsonValue::Object(BTreeMap::new()), 10000);
        if let Err(err) = ping_res {
            process.shutdown();
            return Err(err);
        }

        Ok(process)
    }

    /// Captures any stderr output accumulated by the child process.
    fn read_stderr(&mut self) -> String {
        let mut err_str = String::new();
        if let Some(mut err_stream) = self.child.stderr.take() {
            let _ = err_stream.read_to_string(&mut err_str);
        }
        err_str
    }
}

impl RuntimeBackend for RProcess {
    fn send_request(
        &mut self,
        method: &str,
        params: JsonValue,
        timeout_ms: u64,
    ) -> Result<RpcResponse, RuntimeError> {
        if !self.is_alive {
            let stderr = self.read_stderr();
            return Err(RuntimeError::ProcessTerminated { code: None, stderr });
        }

        let req_id = self.next_id;
        self.next_id += 1;
        let request = RpcRequest::new(req_id, method, params);

        // Write request to child stdin
        if let Err(e) = write_message(&mut self.stdin, &request) {
            self.shutdown();
            return Err(e);
        }

        // Wait for response on receiver channel with timeout
        let duration = Duration::from_millis(timeout_ms);
        match self.resp_receiver.recv_timeout(duration) {
            Ok(Ok(response)) => {
                if response.id != req_id {
                    return Err(RuntimeError::ProtocolError(format!(
                        "Response ID mismatch: expected {}, got {}",
                        req_id, response.id
                    )));
                }
                Ok(response)
            }
            Ok(Err(err)) => {
                self.shutdown();
                Err(err)
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                self.shutdown();
                Err(RuntimeError::Timeout {
                    timeout_ms,
                    operation: method.to_string(),
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                let stderr = self.read_stderr();
                let status = self.child.wait().ok().and_then(|s| s.code());
                self.is_alive = false;
                Err(RuntimeError::ProcessTerminated {
                    code: status,
                    stderr,
                })
            }
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn shutdown(&mut self) {
        if self.is_alive {
            // Attempt clean exit, then kill if needed
            let _ = write_message(
                &mut self.stdin,
                &RpcRequest::new(0, "exit", JsonValue::Object(BTreeMap::new())),
            );
            let _ = self.child.kill();
            let _ = self.child.wait();
            self.is_alive = false;
        }
        if let Some(path) = self.temp_script_path.take() {
            let _ = fs::remove_file(path);
        }
    }
}

impl Drop for RProcess {
    fn drop(&mut self) {
        self.shutdown();
    }
}
