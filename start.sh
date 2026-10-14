#!/usr/bin/env bash
# ==============================================================================
# R-PARSE — One-Command Local Startup (Bash)
#
# Builds and launches the unified R-PARSE web application (Landing Page + Studio)
# on http://localhost:8080 with automatic browser launch.
# ==============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}/implementation"

echo "================================================================="
echo " Starting R-PARSE Unified Web Application & Analysis Engine...   "
echo "================================================================="

cargo run -p rparse-cli -- studio --open
