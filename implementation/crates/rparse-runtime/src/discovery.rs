//! GNU R and Rscript executable discovery across platforms.

use std::env;
use std::path::{Path, PathBuf};

use crate::error::RuntimeError;

/// Locates a valid GNU R or Rscript executable using standard discovery heuristics.
///
/// Discovery precedence:
/// 1. `explicit_override` passed via config or argument.
/// 2. `RPARSE_R_PATH` environment variable.
/// 3. `R_HOME` environment variable (inspects `<R_HOME>/bin/Rscript` or `x64/Rscript`).
/// 4. System `PATH` search for `Rscript` (or `Rscript.exe`).
/// 5. Standard platform-specific installation directories:
///    - Windows: `C:\Program Files\R\R-*\bin\Rscript.exe` or `x64\Rscript.exe`
///    - macOS: `/Library/Frameworks/R.framework/Resources/bin/Rscript`, Homebrew paths
///    - Linux/Unix: `/usr/bin/Rscript`, `/usr/local/bin/Rscript`
pub fn discover_r_executable(explicit_override: Option<&Path>) -> Result<PathBuf, RuntimeError> {
    let mut searched = Vec::new();

    // 1. Explicit override
    if let Some(path) = explicit_override {
        if path.is_file() {
            return Ok(path.to_path_buf());
        } else {
            return Err(RuntimeError::ExecutableNotFound {
                searched: vec![format!("explicit override: {}", path.display())],
            });
        }
    }

    // 2. RPARSE_R_PATH environment variable
    if let Ok(val) = env::var("RPARSE_R_PATH") {
        let path = PathBuf::from(&val);
        searched.push(format!("env RPARSE_R_PATH: {}", val));
        if path.is_file() {
            return Ok(path);
        }
    }

    // 3. R_HOME environment variable
    if let Ok(val) = env::var("R_HOME") {
        let r_home = PathBuf::from(&val);
        searched.push(format!("env R_HOME: {}", val));

        let candidates = [
            r_home.join("bin").join("Rscript.exe"),
            r_home.join("bin").join("x64").join("Rscript.exe"),
            r_home.join("bin").join("Rscript"),
            r_home.join("bin").join("R.exe"),
            r_home.join("bin").join("R"),
        ];
        for candidate in candidates {
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }

    // 4. System PATH lookup
    let binary_names = if cfg!(windows) {
        &["Rscript.exe", "R.exe"][..]
    } else {
        &["Rscript", "R"][..]
    };

    if let Ok(path_var) = env::var("PATH") {
        for dir in env::split_paths(&path_var) {
            for bin_name in binary_names {
                let candidate = dir.join(bin_name);
                if candidate.is_file() {
                    return Ok(candidate);
                }
            }
        }
    }
    searched.push("system PATH".to_string());

    // 5. Platform-specific fallback locations
    #[cfg(windows)]
    {
        let program_files = [
            env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string()),
            env::var("ProgramFiles(x86)").unwrap_or_else(|_| r"C:\Program Files (x86)".to_string()),
        ];

        for base in &program_files {
            let r_root = Path::new(base).join("R");
            searched.push(format!("directory: {}", r_root.display()));
            if let Ok(entries) = std::fs::read_dir(&r_root) {
                let mut version_dirs: Vec<PathBuf> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.is_dir())
                    .collect();
                // Sort descending so newer versions take priority
                version_dirs.sort_by(|a, b| b.cmp(a));

                for dir in version_dirs {
                    let candidates = [
                        dir.join("bin").join("Rscript.exe"),
                        dir.join("bin").join("x64").join("Rscript.exe"),
                        dir.join("bin").join("R.exe"),
                        dir.join("bin").join("x64").join("R.exe"),
                    ];
                    for candidate in candidates {
                        if candidate.is_file() {
                            return Ok(candidate);
                        }
                    }
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let mac_candidates = [
            "/Library/Frameworks/R.framework/Resources/bin/Rscript",
            "/opt/homebrew/bin/Rscript",
            "/usr/local/bin/Rscript",
        ];
        for candidate_str in mac_candidates {
            searched.push(candidate_str.to_string());
            let path = PathBuf::from(candidate_str);
            if path.is_file() {
                return Ok(path);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let linux_candidates = [
            "/usr/bin/Rscript",
            "/usr/local/bin/Rscript",
            "/opt/R/bin/Rscript",
        ];
        for candidate_str in linux_candidates {
            searched.push(candidate_str.to_string());
            let path = PathBuf::from(candidate_str);
            if path.is_file() {
                return Ok(path);
            }
        }
    }

    Err(RuntimeError::ExecutableNotFound { searched })
}

/// Helper returning true if a valid GNU R executable can be discovered.
pub fn is_r_available(explicit_override: Option<&Path>) -> bool {
    discover_r_executable(explicit_override).is_ok()
}
