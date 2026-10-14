//! Structured typed errors for the GNU R runtime bridge.

use std::fmt;
use std::path::PathBuf;

/// Error conditions arising during GNU R process discovery, lifecycle, protocol transport,
/// or code evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    /// GNU R / Rscript executable could not be discovered on the system.
    ExecutableNotFound { searched: Vec<String> },

    /// Failed to spawn the GNU R child process.
    ProcessSpawnFailed { path: PathBuf, message: String },

    /// Operation timed out before completion.
    Timeout { timeout_ms: u64, operation: String },

    /// The GNU R child process terminated unexpectedly or exited with a non-zero code.
    ProcessTerminated { code: Option<i32>, stderr: String },

    /// Unexpected EOF on communication stream.
    UnexpectedEof,

    /// Underlying I/O error occurred.
    IoError(String),

    /// Protocol violation, bad framing, or malformed message payload.
    ProtocolError(String),

    /// JSON serialization or deserialization failure.
    SerializationError(String),

    /// Attempted to execute code or invoke runtime bridge in `AnalysisMode::Static`.
    ExecutionForbiddenInStaticMode,

    /// Execution error reported by GNU R (e.g. syntax error, runtime exception).
    RRuntimeError {
        message: String,
        call: Option<String>,
    },

    /// The target symbol was not found in the evaluation environment.
    ObjectNotFound { symbol: String },

    /// Observation failed to produce a valid schema or result.
    ObservationFailed(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExecutableNotFound { searched } => {
                write!(
                    f,
                    "GNU R executable not found. Searched candidate paths: [{}]",
                    searched.join(", ")
                )
            }
            Self::ProcessSpawnFailed { path, message } => {
                write!(
                    f,
                    "Failed to spawn GNU R process at {}: {}",
                    path.display(),
                    message
                )
            }
            Self::Timeout {
                timeout_ms,
                operation,
            } => {
                write!(
                    f,
                    "Runtime operation '{}' timed out after {} ms",
                    operation, timeout_ms
                )
            }
            Self::ProcessTerminated { code, stderr } => {
                write!(
                    f,
                    "GNU R process terminated with exit code {:?}: {}",
                    code, stderr
                )
            }
            Self::UnexpectedEof => {
                write!(
                    f,
                    "Unexpected end of stream while reading from GNU R process"
                )
            }
            Self::IoError(msg) => write!(f, "Runtime I/O error: {}", msg),
            Self::ProtocolError(msg) => write!(f, "Runtime protocol error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Runtime serialization error: {}", msg),
            Self::ExecutionForbiddenInStaticMode => {
                write!(
                    f,
                    "Runtime execution is strictly forbidden in STATIC analysis mode"
                )
            }
            Self::RRuntimeError { message, call } => {
                if let Some(c) = call {
                    write!(f, "R error in {}: {}", c, message)
                } else {
                    write!(f, "R error: {}", message)
                }
            }
            Self::ObjectNotFound { symbol } => {
                write!(
                    f,
                    "R object '{}' not found in evaluation environment",
                    symbol
                )
            }
            Self::ObservationFailed(msg) => write!(f, "Runtime observation failed: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<std::io::Error> for RuntimeError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}
