#!/bin/zsh
# Zsh Completion Test Harness
# Usage: ./test-zsh-completions.sh [--verbose] [command]

set -euo pipefail

SHHELLY_PATH="${1:-/project}"
VERBOSE=${2:-false}
COMMAND=${3:-docker}

echo "========================================"
echo "Zsh Completion Test Harness"
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
echo "Input: \"$COMMAND run\" (COMP_LINE=\"docker run\", COMP_CWORD=3)"
result=$(zsh -c "COMP_LINE=\"$COMMAND run\" COMP_CWORD=3 compadd \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')")
echo "Completions: $result"
echo ""

# Test with verbose flag if requested
if [ "$VERBOSE" = "true" ]; then
    echo "Test 2: Verbose completion test"
    zsh -x -c "COMP_LINE=\"$COMMAND build\" COMP_CWORD=3 compadd \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')" 2>&1 | tail -50
fi

echo "========================================"
echo "Zsh completion testing complete!"
