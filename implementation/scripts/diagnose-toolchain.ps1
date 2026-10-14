<#
.SYNOPSIS
    R-PARSE Windows Native Build Toolchain Diagnostics
.DESCRIPTION
    Audits the Windows host environment for R-PARSE compilation prerequisites:
    - Rust toolchain and target (x86_64-pc-windows-msvc)
    - Environment variable overrides (CC, CXX, CFLAGS, etc.)
    - Microsoft Visual Studio C++ Build Tools (cl.exe, link.exe)
    - Windows SDK and Universal CRT headers (stdio.h, ucrt.lib)
    - Windows 11 Smart App Control (SAC) enforcement state
    
    This script is read-only and does NOT modify system or user settings.
#>

[CmdletBinding()]
param()

$ErrorActionPreference = 'Continue'
$totalChecks = 0
$passedChecks = 0
$warnChecks = 0
$failChecks = 0

function Write-CheckPass($msg) {
    $script:totalChecks++
    $script:passedChecks++
    Write-Host "  [PASS] $msg" -ForegroundColor Green
}

function Write-CheckWarn($msg, $detail) {
    $script:totalChecks++
    $script:warnChecks++
    Write-Host "  [WARN] $msg" -ForegroundColor Yellow
    if ($detail) {
        Write-Host "         $detail" -ForegroundColor DarkYellow
    }
}

function Write-CheckFail($msg, $detail) {
    $script:totalChecks++
    $script:failChecks++
    Write-Host "  [FAIL] $msg" -ForegroundColor Red
    if ($detail) {
        Write-Host "         $detail" -ForegroundColor DarkRed
    }
}

Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  R-PARSE Windows Native Toolchain Diagnostic Suite" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# -----------------------------------------------------------------------------
# 1. Rust Toolchain and Target
# -----------------------------------------------------------------------------
Write-Host "[1/5] Checking Rust Toolchain..." -ForegroundColor Cyan

$rustc = Get-Command rustc -ErrorAction SilentlyContinue
$cargo = Get-Command cargo -ErrorAction SilentlyContinue
$rustup = Get-Command rustup -ErrorAction SilentlyContinue

if (-not $rustc -or -not $cargo) {
    Write-CheckFail "Rust compiler or Cargo not found in PATH." "Install rustup from https://rustup.rs"
} else {
    $rustcVersion = (& rustc --version) 2>$null
    $cargoVersion = (& cargo --version) 2>$null
    Write-CheckPass "Found $rustcVersion and $cargoVersion"

    if ($rustup) {
        $rustupShow = (& rustup show) 2>$null | Out-String
        if ($rustupShow -match 'stable-x86_64-pc-windows-msvc \(active') {
            Write-CheckPass "Active Rust toolchain is stable-x86_64-pc-windows-msvc"
        } elseif ($rustupShow -match 'x86_64-pc-windows-msvc') {
            Write-CheckWarn "Target x86_64-pc-windows-msvc installed, but verify it is default active toolchain." "Run: rustup default stable-x86_64-pc-windows-msvc"
        } else {
            Write-CheckFail "Target x86_64-pc-windows-msvc is NOT installed." "Run: rustup target add x86_64-pc-windows-msvc"
        }
    }
}

Write-Host ""

# -----------------------------------------------------------------------------
# 2. Environment Variable Overrides
# -----------------------------------------------------------------------------
Write-Host "[2/5] Checking Environment Variable Overrides..." -ForegroundColor Cyan

$problemVars = @()
$envVarsToCheck = @('CC', 'CXX', 'CFLAGS', 'CXXFLAGS', 'INCLUDE', 'LIB', 'RUSTFLAGS')

foreach ($var in $envVarsToCheck) {
    $procVal = [System.Environment]::GetEnvironmentVariable($var, [System.EnvironmentVariableTarget]::Process)
    $userVal = [System.Environment]::GetEnvironmentVariable($var, [System.EnvironmentVariableTarget]::User)
    $machVal = [System.Environment]::GetEnvironmentVariable($var, [System.EnvironmentVariableTarget]::Machine)

    if ($procVal -or $userVal -or $machVal) {
        $val = if ($procVal) { $procVal } elseif ($userVal) { $userVal } else { $machVal }
        $scope = if ($procVal) { "Process" } elseif ($userVal) { "User" } else { "Machine" }
        $problemVars += [PSCustomObject]@{
            Variable = $var
            Scope    = $scope
            Value    = $val
        }
    }
}

if ($problemVars.Count -eq 0) {
    Write-CheckPass "No conflicting CC/CXX/CFLAGS compiler overrides detected."
} else {
    foreach ($pv in $problemVars) {
        if ($pv.Variable -match '^(CC|CXX)$' -and $pv.Value -match '(clang|gcc|mingw)') {
            Write-CheckFail "$($pv.Variable) is set in $($pv.Scope) environment: $($pv.Value)" "Cargo/cc-rs will erroneously invoke this compiler for MSVC targets. Unset this variable."
        } else {
            Write-CheckWarn "$($pv.Variable) is set in $($pv.Scope) environment: $($pv.Value)" "Ensure this does not conflict with native MSVC builds."
        }
    }
}

# Check project-level .cargo/config.toml
$workspaceDir = (Split-Path -Parent $PSScriptRoot)
$cargoConfig = Join-Path $workspaceDir ".cargo\config.toml"
if (Test-Path $cargoConfig) {
    $content = Get-Content $cargoConfig -Raw
    if ($content -match '(?m)^\s*CC\s*=') {
        Write-CheckFail "implementation/.cargo/config.toml overrides CC!" "Remove hardcoded compiler paths from .cargo/config.toml to allow auto-discovery."
    } elseif ($content -match '(?m)^\s*target-dir\s*=\s*"[A-Za-z]:') {
        Write-CheckWarn "implementation/.cargo/config.toml contains hardcoded absolute target-dir." "Consider removing machine-specific target-dir for portability."
    } else {
        Write-CheckPass "implementation/.cargo/config.toml is clean of conflicting compiler overrides."
    }
} else {
    Write-CheckPass "No local .cargo/config.toml overrides (standard Cargo defaults active)."
}

Write-Host ""

# -----------------------------------------------------------------------------
# 3. Visual Studio C++ Build Tools & MSVC Compiler
# -----------------------------------------------------------------------------
Write-Host "[3/5] Checking Visual Studio & MSVC C/C++ Toolset..." -ForegroundColor Cyan

$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
$msvcFound = $false
$clPath = $null
$linkPath = $null

if (Test-Path $vswhere) {
    $vsInstall = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -format json | ConvertFrom-Json
    if ($vsInstall -and $vsInstall.Count -gt 0) {
        $instPath = $vsInstall[0].installationPath
        Write-CheckPass "Found Visual Studio installation at: $instPath"

        $clExes = @(& $vswhere -products * -find '**\Hostx64\x64\cl.exe')
        if ($clExes.Count -gt 0 -and (Test-Path $clExes[0])) {
            $clPath = $clExes[0]
            Write-CheckPass "Found MSVC 64-bit compiler: $clPath"
            $msvcFound = $true
        }

        $linkExes = @(& $vswhere -products * -find '**\Hostx64\x64\link.exe')
        if ($linkExes.Count -gt 0 -and (Test-Path $linkExes[0])) {
            $linkPath = $linkExes[0]
            Write-CheckPass "Found MSVC 64-bit linker: $linkPath"
        }
    }
}

if (-not $msvcFound) {
    Write-CheckFail "MSVC C++ Build Tools (cl.exe / link.exe) not found." "Install Visual Studio Build Tools with workload 'Desktop development with C++' (Microsoft.VisualStudio.Workload.VCTools)."
}

Write-Host ""

# -----------------------------------------------------------------------------
# 4. Windows SDK & Universal CRT Headers
# -----------------------------------------------------------------------------
Write-Host "[4/5] Checking Windows SDK and Standard C Headers..." -ForegroundColor Cyan

$sdkIncludeDir = "C:\Program Files (x86)\Windows Kits\10\Include"
$sdkLibDir = "C:\Program Files (x86)\Windows Kits\10\Lib"
$stdioFound = $false
$ucrtLibFound = $false

if (Test-Path $sdkIncludeDir) {
    $stdioFiles = Get-ChildItem -Path $sdkIncludeDir -Recurse -Filter "stdio.h" -ErrorAction SilentlyContinue
    if ($stdioFiles -and $stdioFiles.Count -gt 0) {
        $stdioFound = $true
        $stdioPath = $stdioFiles[0].FullName
        Write-CheckPass "Found C standard library header stdio.h at: $stdioPath"
    }
}

if (Test-Path $sdkLibDir) {
    $ucrtLibs = Get-ChildItem -Path $sdkLibDir -Recurse -Filter "ucrt.lib" -ErrorAction SilentlyContinue | Where-Object { $_.FullName -match 'x64' }
    if ($ucrtLibs -and $ucrtLibs.Count -gt 0) {
        $ucrtLibFound = $true
        $libPath = $ucrtLibs[0].FullName
        Write-CheckPass "Found Universal CRT library ucrt.lib at: $libPath"
    }
}

if (-not $stdioFound) {
    Write-CheckFail "stdio.h not found in Windows Kits." "Install Windows 10/11 SDK via Visual Studio Installer (individual component 'Windows 11 SDK' or 'Windows 10 SDK')."
}
if (-not $ucrtLibFound) {
    Write-CheckFail "ucrt.lib (x64) not found in Windows Kits." "Ensure Windows SDK includes Universal CRT development libraries."
}

Write-Host ""

# -----------------------------------------------------------------------------
# 5. Windows 11 Smart App Control (SAC)
# -----------------------------------------------------------------------------
Write-Host "[5/5] Checking Windows 11 Smart App Control (SAC)..." -ForegroundColor Cyan

$sacPolicyPath = "HKLM:\SYSTEM\CurrentControlSet\Control\CI\Policy"
$sacState = $null
if (Test-Path $sacPolicyPath) {
    $policy = Get-ItemProperty -Path $sacPolicyPath -ErrorAction SilentlyContinue
    if ($policy -and $policy.PSObject.Properties['VerifiedAndReputablePolicyState']) {
        $sacState = $policy.VerifiedAndReputablePolicyState
    }
}

switch ($sacState) {
    0 {
        Write-CheckPass "Smart App Control is OFF (Standard developer workstation configuration)."
    }
    1 {
        Write-CheckWarn "Smart App Control is ENFORCED (On)." "SAC blocks locally compiled unsigned binaries and test runners (os error 4551 / Event 3118). To develop locally, set Smart App Control to 'Evaluation' or 'Off' in Windows Security > App & browser control > Smart App Control settings."
    }
    2 {
        Write-CheckPass "Smart App Control is in EVALUATION mode (Audit only, does not block unsigned binaries)."
    }
    default {
        Write-CheckPass "Smart App Control registry key not active or OS does not enforce SAC."
    }
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Diagnostic Summary: $passedChecks Passed, $warnChecks Warnings, $failChecks Failures" -ForegroundColor $(if ($failChecks -gt 0) { [ConsoleColor]::Red } elseif ($warnChecks -gt 0) { [ConsoleColor]::Yellow } else { [ConsoleColor]::Green })
Write-Host "============================================================" -ForegroundColor Cyan

if ($failChecks -gt 0) {
    exit 1
} else {
    exit 0
}
