//! Unified error model for the R-PARSE CLI.

use std::fmt;
use std::path::PathBuf;

use crate::exit_code::*;

/// Strongly typed errors encountered during CLI execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    /// Invalid command-line arguments or syntax.
    Usage(String),

    /// File I/O error (file missing, unreadable, directory instead of file).
    InputFile { path: PathBuf, message: String },

    /// Static analysis or parsing failed critically.
    AnalysisFailed(String),

    /// Failed to resolve a column or slice criterion in the analysis model.
    ResolutionFailed(String),

    /// Runtime bridge execution failure.
    RuntimeFailed(String),

    /// Internal error or unexpected state.
    Internal(String),
}

impl CliError {
    /// Returns the standard numeric process exit code for this error.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Usage(_) => EXIT_INVALID_USAGE,
            Self::InputFile { .. } => EXIT_INPUT_ERROR,
            Self::AnalysisFailed(_) => EXIT_ANALYSIS_FAILURE,
            Self::ResolutionFailed(_) => EXIT_ANALYSIS_FAILURE,
            Self::RuntimeFailed(_) => EXIT_ANALYSIS_FAILURE,
            Self::Internal(_) => EXIT_INTERNAL_ERROR,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage(msg) => write!(f, "Usage error: {}", msg),
            Self::InputFile { path, message } => {
                write!(f, "Input file error '{}': {}", path.display(), message)
            }
            Self::AnalysisFailed(msg) => write!(f, "Analysis failed: {}", msg),
            Self::ResolutionFailed(msg) => write!(f, "Resolution error: {}", msg),
            Self::RuntimeFailed(msg) => write!(f, "Runtime error: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}
