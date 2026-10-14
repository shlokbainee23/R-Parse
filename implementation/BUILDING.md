# R-PARSE — Windows Native Build & Toolchain Guide

This document defines the supported Windows native toolchain for the R-PARSE Rust workspace (`implementation/`), details first-time setup, provides diagnostic verification instructions, and documents Windows security configuration for local development.

---

## 1. Supported Toolchain Architecture

R-PARSE uses a hybrid architecture with a standalone Rust core (Tier 1) and an optional GNU R runtime bridge (Tier 2). The native Windows build relies on the **Microsoft MSVC toolchain**:

| Component | Supported Version / Target | Provider |
|---|---|---|
| **Rust Toolchain** | `stable-x86_64-pc-windows-msvc` | `rustup` |
| **Rust Host / Target** | `x86_64-pc-windows-msvc` | Rust compiler (`rustc`) |
| **Native C Compiler** | Microsoft Visual C++ (`cl.exe`, 64-bit) | Visual Studio Build Tools |
| **Native Linker** | Microsoft Linker (`link.exe`, 64-bit) | Visual Studio Build Tools |
| **C Standard Library** | Universal CRT (`ucrt\stdio.h`) | Windows 10/11 SDK |
| **Native C Libraries** | `ucrt.lib`, `kernel32.lib` | Windows 10/11 SDK |

> [!IMPORTANT]
> Do NOT mix toolchains. When Rust targets `x86_64-pc-windows-msvc`, native C dependencies (such as `tree-sitter` and `tree-sitter-r`) must be compiled using the Microsoft MSVC compiler (`cl.exe`) against the Windows SDK Universal CRT headers. Forcing an external GCC or MinGW Clang compiler via `CC` while targeting MSVC causes fatal header lookup errors (`fatal error: 'stdio.h' file not found`).

---

## 2. Prerequisites & First-Time Setup

### A. Rust Toolchain Setup

1. Install `rustup` from [https://rustup.rs](https://rustup.rs).
2. Set the default host and active toolchain to MSVC:
   ```powershell
   rustup default stable-x86_64-pc-windows-msvc
   rustup target add x86_64-pc-windows-msvc
   ```
3. Confirm installation:
   ```powershell
   rustc -vV
   cargo -vV
   ```
   Both commands should report `host: x86_64-pc-windows-msvc`.

### B. Microsoft Visual Studio C++ Build Tools & Windows SDK

1. Download the **Visual Studio Build Tools** installer from [Visual Studio Downloads](https://visualstudio.microsoft.com/visual-cpp-build-tools/) or install via winget:
   ```powershell
   winget install Microsoft.VisualStudio.2022.BuildTools --override "--passive --wait --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 --add Microsoft.VisualStudio.Component.Windows11SDK.26100"
   ```
2. Required workloads and individual components:
   - **Workload:** "Desktop development with C++" (`Microsoft.VisualStudio.Workload.VCTools`)
   - **MSVC v143/v144/v145 x64/x86 build tools:** `Microsoft.VisualStudio.Component.VC.Tools.x86.x64`
   - **Windows SDK:** Windows 11 SDK (e.g., `10.0.26100.0` or `10.0.22621.0`) or Windows 10 SDK (`Microsoft.VisualStudio.Component.Windows11SDK.26100`)

### C. Environment Hygiene & Configuration Rules

- **Do NOT set `CC`, `CXX`, or `CFLAGS`**: Cargo and the `cc` crate automatically discover `cl.exe` and Windows SDK paths through Visual Studio locator mechanisms (`vswhere`).
- **No machine-specific paths in `.cargo/config.toml`**: The workspace does not require any local `.cargo/config.toml` file. Hardcoded compiler overrides or hardcoded absolute target directories break portability across developer machines.

---

## 3. Windows 11 Smart App Control (SAC)

On Windows 11, **Smart App Control (SAC)** is a security feature that uses Microsoft cloud reputation to block unsigned or unrecognized binaries (`VerifiedAndReputablePolicyState = 1`).

### Impact on Software Development
When Smart App Control is **On (Enforced)**:
- Cargo build scripts (`build-script-build.exe`), test runner binaries (`target\debug\deps\*.exe`), and freshly compiled executables (`rparse.exe`) are unsigned local binaries without cloud reputation.
- Windows Code Integrity policy `{0283ac0f-fff1-49ae-ada1-8a933130cad6}` blocks them with:
  ```text
  An Application Control policy has blocked this file. (os error 4551)
  ```
- Windows Event Log (`Microsoft-Windows-CodeIntegrity/Operational`) logs Event IDs **3118**, **3077**, or **3033** ("Smart App Control Block Details").

### Required Developer Configuration
Microsoft requires that workstations used for active software development have Smart App Control configured to **Evaluation** or **Off**:

1. Open **Windows Security** (Press `Win + R`, enter `windowsdefender://appbrowser`).
2. Click **App & browser control**.
3. Click **Smart App Control settings**.
4. Select **Evaluation** or **Off**.

---

## 4. Automated Toolchain Diagnostics

R-PARSE includes an automated diagnostic script that audits all prerequisites without making any system changes.

From the `implementation/` directory, run:

```powershell
powershell -ExecutionPolicy Bypass -File scripts/diagnose-toolchain.ps1
```

The script verifies:
1. `rustc` and `cargo` availability with active `x86_64-pc-windows-msvc` target.
2. Absence of conflicting `CC`, `CXX`, `CFLAGS`, and `.cargo/config.toml` compiler overrides.
3. Microsoft Visual Studio installation, 64-bit `cl.exe` compiler, and `link.exe` linker.
4. Windows SDK installation, C standard library header `stdio.h`, and `ucrt.lib`.
5. Windows 11 Smart App Control (SAC) state.

---

## 5. Standard Verification & Build Commands

All commands should be executed from the `implementation/` directory:

```powershell
# Check code formatting across workspace
cargo fmt --all --check

# Compile and check all workspace crates (including C dependencies)
cargo check --workspace

# Run all unit and integration tests across all crates
cargo test --workspace

# Run Clippy linter across all targets and features
cargo clippy --workspace --all-targets --all-features

# Build optimized release binaries
cargo build --workspace --release
```

### Validating the Release CLI Binary

After a release build, validate the binary:

```powershell
# Show version
.\target\release\rparse.exe --version

# Show help
.\target\release\rparse.exe --help

# Run static analysis summary on canonical fixture
.\target\release\rparse.exe analyze fixtures\canonical.R

# Run fine-grained column provenance
.\target\release\rparse.exe provenance fixtures\canonical.R --column "df$total"

# Run backward static slice for column criterion
.\target\release\rparse.exe slice fixtures\canonical.R --criterion "df$total"
```

### Clean Rebuild

To perform a clean rebuild using standard Cargo workflows:

```powershell
cargo clean
cargo build --workspace --release
```
