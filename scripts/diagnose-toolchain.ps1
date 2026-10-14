# Wrapper script forwarding to implementation/scripts/diagnose-toolchain.ps1
$scriptPath = Join-Path $PSScriptRoot "..\implementation\scripts\diagnose-toolchain.ps1"
if (-not (Test-Path $scriptPath)) {
    $scriptPath = Join-Path $PSScriptRoot "implementation\scripts\diagnose-toolchain.ps1"
}
& $scriptPath @args
