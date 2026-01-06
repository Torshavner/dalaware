#!/usr/bin/env bash
set -euo pipefail

echo "==> Stopping database container..."
docker-compose -f docker/docker-compose.yml stop postgres-timescale

echo "==> Removing database volume..."
docker-compose -f docker/docker-compose.yml rm -f postgres-timescale
docker volume rm dreadnought_pgdata 2>/dev/null || true

echo "==> Starting fresh database..."
docker-compose -f docker/docker-compose.yml up -d postgres-timescale

echo "==> Waiting for database to be ready..."
sleep 5

echo "==> Database reset complete!"
echo "    Connection: postgresql://postgres:password@localhost:5432/main_db"
