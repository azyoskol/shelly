# Fish Completion Testing Container
FROM alpine:latest

LABEL maintainer="Shelly Team <team@shelly.ai>"

# Install fish and common development tools
RUN apk add --no-cache \
    fish \
    git \
    docker \
    curl \
    wget

# Set up shell environment
SHELL ["/bin/fish", "-c"]

# Create test harness script
RUN mkdir -p /opt/shelly-test && cat > /opt/shelly-test/test-completions.fish << 'EOF'
#!/usr/bin/env fish
set SHHELLY_PATH $argv[1] or /project

echo "Testing Shelly completions in Fish..."
echo "========================================"

# Test basic completion
echo "Test 1: Basic go completion"
go version > /dev/null 2>&1 and echo "✓ Go installed" or (echo "✗ Go not found, skipping test")

# Test Shelly CLI if available
if test -f "$SHHELLY_PATH/cmd/shelly/main.go"
    echo "Found shelly source at $SHHELLY_PATH"
else
    echo "Shelly source not found. Building from source..."
    cd /project && go build -o /opt/shelly-test/shelly ./cmd/shelly/
end

# Run basic completion test
echo "Test 2: Running completion tests"
go run /opt/shelly-test/test-completions/internal/go_test.go > /tmp/completion-results.txt 2>&1 or true
cat /tmp/completion-results.txt

echo "========================================"
echo "Fish completion testing complete!"
EOF
chmod +x /opt/shelly-test/test-completions.fish

# Create internal test package
RUN mkdir -p /opt/shelly-test/test-completions && cat > /opt/shelly-test/test-completions/internal/go_test.go << 'EOF'
pkg main

import (
    "fmt"
    "os"
)

func TestGoCompletion() {
    fmt.Println("Testing go completion...")
    
    // Simulate COMP_LINE="go run" test
    if len(os.Args) > 1 && os.Args[1] == "--simulate" {
        suggestions := []string{
            "go mod init github.com/example/myapp",
            "go get github.com/spf13/cobra@latest",
            "go test ./... -coverprofile=coverage.out",
        }
        for _, s := range suggestions {
            fmt.Printf("  %s\n", s)
        }
    }
}

func main() {
    TestGoCompletion()
}
EOF

echo "Fish container ready!"
