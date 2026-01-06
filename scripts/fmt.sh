#!/usr/bin/env bash
set -euo pipefail

echo "==> Running cargo fmt..."
cargo fmt --all -- --check

echo "==> Format check complete!"
