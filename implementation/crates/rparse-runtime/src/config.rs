//! Configuration and execution modes for the GNU R runtime bridge.

use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

/// Default evaluation timeout in milliseconds.
pub const DEFAULT_TIMEOUT_MS: u64 = 2000;

/// Execution and analysis mode controlling Tier 2 runtime invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum AnalysisMode {
    /// Static analysis only. Strictly forbidden from launching or invoking GNU R.
    /// 100% offline, deterministic, zero side-effects.
    #[default]
    Static,
    /// Static analysis is primary. Selective, bounded on-demand queries to GNU R
    /// to observe runtime schemas and refine unresolvable boundaries.
    Assisted,
    /// Explicit script execution via GNU R to produce empirical oracle traces
    /// and comprehensive runtime observations.
    Execute,
}

impl AnalysisMode {
    /// Returns true if this mode strictly forbids runtime execution.
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static)
    }

    /// Returns true if this mode permits on-demand bounded runtime assistance.
    pub fn is_assisted(&self) -> bool {
        matches!(self, Self::Assisted)
    }

    /// Returns true if this mode performs execution.
    pub fn is_execute(&self) -> bool {
        matches!(self, Self::Execute)
    }

    /// Returns true if runtime execution is permitted in this mode.
    pub fn allows_execution(&self) -> bool {
        matches!(self, Self::Assisted | Self::Execute)
    }
}

impl fmt::Display for AnalysisMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Static => write!(f, "STATIC"),
            Self::Assisted => write!(f, "ASSISTED"),
            Self::Execute => write!(f, "EXECUTE"),
        }
    }
}

/// Configuration options for initializing and driving the GNU R runtime bridge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// The active analysis mode. Defaults to `AnalysisMode::Static`.
    pub mode: AnalysisMode,
    /// Optional explicit path to the GNU R / Rscript executable.
    pub r_executable: Option<PathBuf>,
    /// Execution timeout per operation in milliseconds. Defaults to 2000 ms.
    pub timeout_ms: u64,
    /// Optional controlled working directory for child evaluation processes.
    pub work_dir: Option<PathBuf>,
    /// Controlled environment variables passed to the child process.
    pub env_vars: BTreeMap<String, String>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            mode: AnalysisMode::Static,
            r_executable: None,
            timeout_ms: DEFAULT_TIMEOUT_MS,
            work_dir: None,
            env_vars: BTreeMap::new(),
        }
    }
}

impl RuntimeConfig {
    /// Creates a new default configuration in `AnalysisMode::Static`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the analysis mode.
    pub fn with_mode(mut self, mode: AnalysisMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets an explicit GNU R executable path override.
    pub fn with_r_executable(mut self, path: impl Into<PathBuf>) -> Self {
        self.r_executable = Some(path.into());
        self
    }

    /// Sets the execution timeout in milliseconds.
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Sets the controlled working directory.
    pub fn with_work_dir(mut self, path: impl Into<PathBuf>) -> Self {
        self.work_dir = Some(path.into());
        self
    }

    /// Adds a controlled environment variable.
    pub fn with_env_var(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.env_vars.insert(key.into(), value.into());
        self
    }
}
