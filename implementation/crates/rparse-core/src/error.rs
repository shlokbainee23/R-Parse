//! Core error types and Result definitions for R-PARSE.

use std::path::PathBuf;
use thiserror::Error;

/// The primary error type for R-PARSE core and parser operations.
#[derive(Debug, Error)]
pub enum RParseError {
    /// An I/O error occurred while reading source files or writing outputs.
    #[error("I/O error at {path:?}: {source}")]
    Io {
        /// Associated file path if known.
        path: Option<PathBuf>,
        /// Underlying standard I/O error.
        #[source]
        source: std::io::Error,
    },

    /// The source input contains invalid UTF-8 bytes.
    #[error("Source is not valid UTF-8: {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    /// A parser failure or configuration issue occurred.
    #[error("Parser error: {message}")]
    Parser {
        /// Parser error description.
        message: String,
    },

    /// An internal invariant was violated.
    #[error("Internal R-PARSE error: {0}")]
    Internal(String),
}

impl RParseError {
    /// Creates an I/O error associated with a specific path.
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: Some(path.into()),
            source,
        }
    }

    /// Creates an I/O error without a specific path.
    pub fn io_generic(source: std::io::Error) -> Self {
        Self::Io { path: None, source }
    }

    /// Creates a parser-level error.
    pub fn parser(message: impl Into<String>) -> Self {
        Self::Parser {
            message: message.into(),
        }
    }

    /// Creates an internal error.
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

/// A specialized [`Result`] type for R-PARSE operations.
pub type Result<T> = std::result::Result<T, RParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = RParseError::parser("Failed to initialize grammar");
        assert_eq!(
            err.to_string(),
            "Parser error: Failed to initialize grammar"
        );
    }
}
