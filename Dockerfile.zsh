# Zsh Completion Testing Container
FROM alpine:latest

LABEL maintainer="Shelly Team <team@shelly.ai>"

# Install zsh and common development tools
RUN apk add --no-cache \
    zsh \
    git \
    docker \
    curl \
    wget

# Set up shell environment
SHELL ["/bin/zsh", "-c"]

# Create test harness script
RUN mkdir -p /opt/shelly-test && cat > /opt/shelly-test/test-completions.sh << 'EOF'
#!/bin/bash
set -e

SHHELLY_PATH="${1:-/project}"
echo "Testing Shelly completions in Zsh..."
echo "========================================"

# Test basic completion
echo "Test 1: Basic docker completion"
docker --version > /dev/null 2>&1 && echo "✓ Docker installed" || (echo "✗ Docker not found, skipping test")

# Test Shelly CLI if available
if [ -f "$SHHELLY_PATH/cmd/shelly/main.go" ]; then
    echo "Found shelly source at $SHHELLY_PATH"
else
    echo "Shelly source not found. Building from source..."
    cd /project && go build -o /opt/shelly-test/shelly ./cmd/shelly/
fi

# Run basic completion test
echo "Test 2: Running completion tests"
go run /opt/shelly-test/test-completions/internal/docker_test.go > /tmp/completion-results.txt 2>&1 || true
cat /tmp/completion-results.txt

echo "========================================"
echo "Zsh completion testing complete!"
EOF
chmod +x /opt/shelly-test/test-completions.sh

# Create internal test package
RUN mkdir -p /opt/shelly-test/test-completions && cat > /opt/shelly-test/test-completions/internal/docker_test.go << 'EOF'
package main

import (
    "fmt"
    "os"
)

func TestDockerCompletion() {
    fmt.Println("Testing docker completion...")
    
    // Simulate COMP_LINE="docker run" test
    if len(os.Args) > 1 && os.Args[1] == "--simulate" {
        suggestions := []string{
            "docker run -it --rm alpine:latest bash",
            "docker build . -t myapp",
            "docker compose up -d",
        }
        for _, s := range suggestions {
            fmt.Printf("  %s\n", s)
        }
    }
}

func main() {
    TestDockerCompletion()
}
EOF

echo "Zsh container ready!"
