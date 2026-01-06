#!/usr/bin/env bash
set -euo pipefail

echo "==> Running tests..."
cargo test --workspace --all-features

echo "==> Tests complete!"
