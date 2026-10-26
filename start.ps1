# ==============================================================================
# R-PARSE — One-Command Local Startup (PowerShell)
#
# Builds and launches the unified R-PARSE web application (Landing Page + Studio)
# on http://localhost:8080 with automatic browser launch.
# ==============================================================================

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
Set-Location "$scriptDir\implementation"

$studioUrl = "http://localhost:8080/studio"

function Test-StudioUp {
    try {
        $response = Invoke-WebRequest -Uri "http://127.0.0.1:8080/api/health" -UseBasicParsing -TimeoutSec 2
        return $response.StatusCode -eq 200
    } catch {
        return $false
    }
}

if (Test-StudioUp) {
    Start-Process $studioUrl
    exit 0
}

Get-Process rparse -ErrorAction SilentlyContinue | Stop-Process -Force
Start-Sleep -Seconds 1

Write-Host "=================================================================" -ForegroundColor Cyan
Write-Host " Starting R-PARSE Unified Web Application & Analysis Engine...   " -ForegroundColor Cyan
Write-Host "=================================================================" -ForegroundColor Cyan

cargo run -p rparse-cli -- studio --open
