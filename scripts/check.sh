#!/usr/bin/env bash
set -euo pipefail

echo "==> Running cargo check..."
cargo check --workspace --all-targets --all-features

echo "==> Check complete!"
