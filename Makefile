# Shelly - Intelligent Shell Completion Assistant
# Build, test, and release automation

.PHONY: all build clean test lint format help

.DEFAULT_GOAL := all

# Variables
SHHELLY_PATH ?= .
GO_BIN ?= go
DOCKER_COMPOSE ?= docker-compose
VERBOSE ?= false

# Default target
all:
	@echo "========================================"
	@echo "Shelly Build & Test Suite"
	@echo "========================================"
	@echo "Running all targets..."
	@make build
	@make test
	@make lint
	@make format

# Build the Shelly binary
build:
	@echo "Building Shelly binary..."
	$(GO_BIN) mod download
	$(GO_BIN) build -o shelly ./cmd/shelly/
	@echo "Build complete: ./shelly"

# Run tests with coverage
.test:
	@echo "Running tests with coverage..."
	CGO_ENABLED=0 GOOS=linux $(GO_BIN) test -race -coverprofile=coverage.out -covermode=atomic ./...
	@echo "Tests completed. Coverage report: coverage.out"

test:
	@make .test

# Lint code with golangci-lint
lint:
	@echo "Running linters..."
	golangci-lint run --timeout 5m ./...

# Format code with gofmt and goimports
format:
	@echo "Formatting code..."
	gofmt -s -w .
goimports -w .

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf shelly coverage.out *.out
	rm -rf ./bin/
	rm -rf ./dist/

# Help message
help:
	@echo "Shelly Makefile targets:"
	@echo "  make all     - Build, test, lint, and format (default)"
	@echo "  make build   - Build the Shelly binary"
	@echo "  make test    - Run tests with coverage"
	@echo "  make lint    - Run linters"
	@echo "  make format  - Format code with gofmt and goimports"
	@echo "  make clean   - Clean build artifacts"
	@echo ""
	@echo "Usage: make [target] [--verbose]"

# Verbose mode override
ifneq (,$(filter --verbose, $(MAKEFLAGS)))
	export V=1
endif
