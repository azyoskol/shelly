#!/bin/bash
# Bash Completion Test Harness
# Usage: ./test-bash-completions.sh [--verbose] [command]

set -euo pipefail

SHHELLY_PATH="${1:-/project}"
VERBOSE=${2:-false}
COMMAND=${3:-git}

echo "========================================"
echo "Bash Completion Test Harness"
echo "========================================"
echo "Testing completions for: $COMMAND"
echo "Shelly path: $SHHELLY_PATH"
echo ""

# Check if shelly is built
if [ -f "$SHHELLY_PATH/cmd/shelly/main.go" ]; then
    echo "Building Shelly from source..."
    cd "$SHHELLY_PATH"
    go build -o /opt/shelly-test/shelly ./cmd/shelly/
else
    echo "Shelly not found at $SHHELLY_PATH. Skipping build." >&2
    exit 1
fi

# Test basic completion
echo "Test 1: Basic completion test"
echo "Input: \"$COMMAND cl\" (COMP_LINE=\"git cl\", COMP_CWORD=3)"
result=$(bash -c "COMP_LINE=\"$COMMAND cl\" COMP_CWORD=3 compgen -W \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')")
echo "Completions: $result"
echo ""

# Test with verbose flag if requested
if [ "$VERBOSE" = "true" ]; then
    echo "Test 2: Verbose completion test"
    bash -x -c "COMP_LINE=\"$COMMAND run\" COMP_CWORD=3 compgen -W \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')" 2>&1 | tail -50
fi

echo "========================================"
echo "Bash completion testing complete!"
