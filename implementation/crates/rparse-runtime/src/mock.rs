//! Mock runtime backend for deterministic offline protocol, error, and timeout testing.

use std::collections::{BTreeMap, VecDeque};
use std::thread;
use std::time::Duration;

use crate::error::RuntimeError;
use crate::json::JsonValue;
use crate::process::RuntimeBackend;
use crate::protocol::{RpcError, RpcResponse};

/// A mock backend capable of returning preconfigured responses or simulating failures.
pub struct MockBackend {
    responses: VecDeque<Result<RpcResponse, RuntimeError>>,
    delay: Option<Duration>,
    is_alive: bool,
}

impl Default for MockBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MockBackend {
    /// Creates a new mock backend with an empty response queue.
    pub fn new() -> Self {
        Self {
            responses: VecDeque::new(),
            delay: None,
            is_alive: true,
        }
    }

    /// Enqueues a successful result payload for the next request.
    pub fn queue_result(&mut self, result: JsonValue) {
        self.responses.push_back(Ok(RpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 0, // will be matched to request id
            result: Some(result),
            error: None,
        }));
    }

    /// Enqueues an RPC error response.
    pub fn queue_rpc_error(&mut self, code: i32, message: impl Into<String>) {
        self.responses.push_back(Ok(RpcResponse {
            jsonrpc: "2.0".to_string(),
            id: 0,
            result: None,
            error: Some(RpcError {
                code,
                message: message.into(),
                data: None,
            }),
        }));
    }

    /// Enqueues a runtime error (e.g. protocol error, termination).
    pub fn queue_error(&mut self, err: RuntimeError) {
        self.responses.push_back(Err(err));
    }

    /// Sets an artificial delay before responding (useful for timeout tests).
    pub fn set_delay(&mut self, delay: Duration) {
        self.delay = Some(delay);
    }

    /// Simulates a terminated process.
    pub fn simulate_termination(&mut self) {
        self.is_alive = false;
    }
}

impl RuntimeBackend for MockBackend {
    fn send_request(
        &mut self,
        _method: &str,
        _params: JsonValue,
        timeout_ms: u64,
    ) -> Result<RpcResponse, RuntimeError> {
        if !self.is_alive {
            return Err(RuntimeError::ProcessTerminated {
                code: Some(1),
                stderr: "Mock process terminated".to_string(),
            });
        }

        if let Some(delay) = self.delay {
            if delay > Duration::from_millis(timeout_ms) {
                thread::sleep(Duration::from_millis(timeout_ms + 10));
                return Err(RuntimeError::Timeout {
                    timeout_ms,
                    operation: _method.to_string(),
                });
            } else {
                thread::sleep(delay);
            }
        }

        match self.responses.pop_front() {
            Some(Ok(mut resp)) => {
                resp.id = 1;
                Ok(resp)
            }
            Some(Err(err)) => Err(err),
            None => {
                if _method == "ping" {
                    let mut map = BTreeMap::new();
                    map.insert("status".to_string(), JsonValue::String("pong".to_string()));
                    Ok(RpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: 1,
                        result: Some(JsonValue::Object(map)),
                        error: None,
                    })
                } else {
                    Err(RuntimeError::ProtocolError(
                        "No mock responses available in queue".to_string(),
                    ))
                }
            }
        }
    }

    fn is_alive(&self) -> bool {
        self.is_alive
    }

    fn shutdown(&mut self) {
        self.is_alive = false;
        self.responses.clear();
    }
}
