#!/usr/bin/env bash
set -euo pipefail

echo "==> Starting Docker infrastructure..."
docker-compose -f docker/docker-compose.yml up -d

echo ""
echo "==> Services started:"
echo "    Postgres:  localhost:5432"
echo "    pgAdmin:   http://localhost:8081"
echo "    Grafana:   http://localhost:3000 (admin/admin)"
echo "    Loki:      http://localhost:3100"
echo "    Vector:    UDP 9000 (logs)"
