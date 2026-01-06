#!/usr/bin/env bash
set -euo pipefail

echo "==> Stopping Docker infrastructure..."
docker-compose -f docker/docker-compose.yml down

echo "==> Infrastructure stopped!"
