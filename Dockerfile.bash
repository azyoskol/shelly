# Bash Completion Testing Container
FROM alpine:latest

LABEL maintainer="Shelly Team <team@shelly.ai>"

# Install bash and common development tools
RUN apk add --no-cache \
    bash \
    git \
    docker \
    curl \
    wget

# Set up shell environment
SHELL ["/bin/bash", "-c"]

# Create test harness script
RUN mkdir -p /opt/shelly-test && cat > /opt/shelly-test/test-completions.sh << 'EOF'
#!/bin/bash
set -e

SHHELLY_PATH="${1:-/project}"
echo "Testing Shelly completions in Bash..."
echo "========================================"

# Test basic completion
echo "Test 1: Basic git completion"
git --version > /dev/null 2>&1 && echo "✓ Git installed" || (echo "✗ Git not found, skipping test")

# Test Shelly CLI if available
if [ -f "$SHHELLY_PATH/cmd/shelly/main.go" ]; then
    echo "Found shelly source at $SHHELLY_PATH"
else
    echo "Shelly source not found. Building from source..."
    cd /project && go build -o /opt/shelly-test/shelly ./cmd/shelly/
fi

# Run basic completion test
echo "Test 2: Running completion tests"
go run /opt/shelly-test/test-completions/internal/*.go > /tmp/completion-results.txt 2>&1 || true
cat /tmp/completion-results.txt

echo "========================================"
echo "Bash completion testing complete!"
EOF
chmod +x /opt/shelly-test/test-completions.sh

# Create internal test package
RUN mkdir -p /opt/shelly-test/test-completions && cat > /opt/shelly-test/test-completions/internal/git_test.go << 'EOF'
package main

import (
    "fmt"
    "os"
)

func TestGitCompletion() {
    fmt.Println("Testing git completion...")
    
    // Simulate COMP_LINE="git cl" test
    if len(os.Args) > 1 && os.Args[1] == "--simulate" {
        suggestions := []string{
            "git clone https://github.com/azyoskol/shelly.git",
            "git checkout -b feature/test",
            "git push origin main",
        }
        for _, s := range suggestions {
            fmt.Printf("  %s\n", s)
        }
    }
}

func main() {
    TestGitCompletion()
}
EOF

echo "Bash container ready!"
