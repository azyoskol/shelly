#!/usr/bin/env fish
# Fish Completion Test Harness
# Usage: ./test-fish-completions.sh [--verbose] [command]

set -u
SHHELLY_PATH = $argv[1] or /project
VERBOSE = $argv[2] or false
COMMAND = $argv[3] or go

echo "========================================"
echo "Fish Completion Test Harness"
echo "========================================"
echo "Testing completions for: $COMMAND"
echo "Shelly path: $SHHELLY_PATH"
echo ""

# Check if shelly is built
if test -f "$SHHELLY_PATH/cmd/shelly/main.go"
    echo "Building Shelly from source..."
    cd "$SHHELLY_PATH"
    go build -o /opt/shelly-test/shelly ./cmd/shelly/
else
    echo "Shelly not found at $SHHELLY_PATH. Skipping build." >&2
    exit 1
end

# Test basic completion
echo "Test 1: Basic completion test"
echo "Input: \"$COMMAND mod\" (COMP_LINE=\"go mod\", COMP_CWORD=3)"
result = fish -c "complete -c $COMMAND -a \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')"
echo "Completions: $result"
echo ""

# Test with verbose flag if requested
if test "$VERBOSE" = "true"
    echo "Test 2: Verbose completion test"
    fish -c "complete -c $COMMAND -a \$(/opt/shelly-test/shelly 2>/dev/null || echo 'no-suggestions')" | tail -50
end

echo "========================================"
echo "Fish completion testing complete!"
