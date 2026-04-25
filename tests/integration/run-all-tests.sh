#!/bin/bash
# Integration Test Runner for Shelly Shell Completions
# Usage: ./run-all-tests.sh [--verbose]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SHHELLY_PATH="${1:-/project}"
VERBOSE=${2:-false}
echo "========================================"
echo "Shelly Integration Test Runner"
echo "========================================"
echo "Shelly path: $SHHELLY_PATH"
echo "Verbose mode: $VERBOSE"
echo ""

# Check if docker-compose is available
if ! command -v docker-compose &> /dev/null; then
    echo "ERROR: docker-compose not found. Please install it." >&2
    exit 1
fi

# Build and start all shell testing containers
echo "Building Docker images..."
docker-compose build --no-cache

echo "Starting containers..."
docker-compose up -d

echo "Waiting for containers to be ready..."
sleep 5

# Run tests in each container
echo "========================================"
echo "Running Bash completion tests..."
echo "========================================"
docker exec shelly-bash /opt/shelly-test/test-completions.sh --bash git

echo ""
echo "========================================"
echo "Running Zsh completion tests..."
echo "========================================"
docker exec shelly-zsh /opt/shelly-test/test-completions.sh --zsh docker

echo ""
echo "========================================"
echo "Running Fish completion tests..."
echo "========================================"
docker exec shelly-fish /opt/shelly-test/test-completions.sh --fish go

echo ""
echo "========================================"
echo "Integration testing complete!"
echo "========================================"
