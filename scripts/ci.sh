#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "========================================"
echo "Running CI Pipeline"
echo "========================================"

"${SCRIPT_DIR}/fmt.sh"
echo ""

"${SCRIPT_DIR}/clippy.sh"
echo ""

"${SCRIPT_DIR}/test.sh"
echo ""

"${SCRIPT_DIR}/build.sh"
echo ""

echo "========================================"
echo "CI Pipeline Complete!"
echo "========================================"
