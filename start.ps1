# ==============================================================================
# R-PARSE — One-Command Local Startup (PowerShell)
#
# Builds and launches the unified R-PARSE web application (Landing Page + Studio)
# on http://localhost:8080 with automatic browser launch.
# ==============================================================================

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "$scriptDir\implementation"

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " Starting R-PARSE Unified Web Application & Analysis Engine...   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

cargo run -p rparse-cli -- studio --open
