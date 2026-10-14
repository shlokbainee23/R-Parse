//! Standard exit codes for the R-PARSE CLI.

/// Operation succeeded without error.
pub const EXIT_SUCCESS: i32 = 0;

/// Static analysis or runtime evaluation failed.
pub const EXIT_ANALYSIS_FAILURE: i32 = 1;

/// Invalid command-line usage or arguments.
pub const EXIT_INVALID_USAGE: i32 = 2;

/// File or input error (missing file, unreadable, permission denied).
pub const EXIT_INPUT_ERROR: i32 = 3;

/// Unexpected internal error or panic recovery.
pub const EXIT_INTERNAL_ERROR: i32 = 4;
