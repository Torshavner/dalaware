#!/usr/bin/env bash
set -euo pipefail

SERVICE="${1:-}"

if [ -z "$SERVICE" ]; then
    echo "Usage: $0 <service>"
    echo ""
    echo "Available services:"
    docker-compose -f docker/docker-compose.yml ps --services
    exit 1
fi

docker-compose -f docker/docker-compose.yml logs -f "$SERVICE"
