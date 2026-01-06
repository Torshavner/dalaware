#!/usr/bin/env bash
set -euo pipefail

echo "==> Building workspace..."
cargo build --workspace --all-features

echo "==> Build complete!"
