#!/usr/bin/env bash
set -euo pipefail

echo "==> Installing cargo-llvm-cov if needed..."
cargo install cargo-llvm-cov --quiet 2>/dev/null || true

echo "==> Running tests with coverage..."
cargo llvm-cov --workspace --all-features --html

echo ""
echo "==> Coverage report generated:"
echo "    Open: target/llvm-cov/html/index.html"
