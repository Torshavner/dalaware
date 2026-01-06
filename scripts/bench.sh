#!/usr/bin/env bash
set -euo pipefail

echo "==> Running benchmarks..."
cargo bench --workspace

echo "==> Benchmark results in target/criterion/"
