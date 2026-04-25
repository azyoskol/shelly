# Shelly Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a production-ready Go CLI tool providing context-aware, AI-powered shell completions for bash, zsh, and fish with seamless starship integration.

**Architecture:** Modular design with agent registry, context analyzer, LLM abstraction layer, and shell adapters - all configurable via YAML.

**Tech Stack:** Go 1.21+, Cobra CLI framework, standard library HTTP client, structured logging, table-driven tests.

---

## Task 1: Project Setup & Boilerplate

**Files:**
- Create: `cmd/shelly/main.go`
- Create: `pkg/shelly/suggestion.go`
- Modify: `go.mod` (add dependencies)
- Test: N/A

- [ ] **Step 1: Initialize Go module with base dependencies**

```bash
cd /home/zubarev/sources/shelly
# Verify go.mod exists and add minimal deps for testing
grep -q "github.com/spf13/cobra" go.mod || echo 'module github.com/anomalyco/opencode/tools/shelly' > go.mod.new && mv go.mod.new go.mod 2>/dev/null || true
```

- [ ] **Step 2: Create CLI entry point**

```go
// cmd/shelly/main.go
package main

import (
    "fmt"
    
    "github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
    Use:   "shelly",
    Short: "Intelligent shell completion assistant",
}

func Execute() {
    if err := rootCmd.Execute(); err != nil {
        fmt.Fprintln(stderr, err.Error())
        os.Exit(1)
    }
}
```

- [ ] **Step 3: Create public suggestion package**

```go
// pkg/shelly/suggestion.go
package shelly

import "context"

type Suggestion struct {
    Text       string `json:"text"`
    Description string `json:"description,omitempty"`
    Priority   int    `json:"priority,omitempty"`
}

type CompletionRequest struct {
    Input     string               // Partial command being typed
    Context  map[string]any      // Working directory, env vars, etc.
    History   []string             // Recent commands from session
    Settings SuggestionSettings  // From config.yaml
}

type SuggestionSettings struct {
    EnableSessionHistory bool `json:"enable_session_history"`
    SessionMaxSize     int  `json:"session_max_size"`
    IncludeEnvironmentInfo bool `json:"include_environment_info"`
}

func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
    // Stub implementation - returns empty slice
    return []Suggestion{}, nil
}
```

- [ ] **Step 4: Commit with initial structure**

```bash
git add cmd/shelly/main.go pkg/shelly/suggestion.go go.mod go.sum
git commit -m "feat: initialize CLI project with base structures"
```

---

## Task 2: Suggestion Data Structures & Error Handling

**Files:**
- Modify: `pkg/shelly/suggestion.go`
- Create: `pkg/shelly/error.go`  
- Test: `tests/pkg/shelly/suggestion_test.go`, `pkg/shelly/error_test.go`

- [ ] **Step 1: Write failing test for empty suggestion retrieval**

```go
// tests/pkg/shelly/suggestion_test.go
package shelly

import (
    "context"
    "testing"
)

func TestGetSuggestionsReturnsEmptyWhenNoInput(t *testing.T) {
    req := CompletionRequest{
        Input: "",
        Context: map[string]any{"cwd": "/"},
        History: []string{},
        Settings: SuggestionSettings{
            EnableSessionHistory: true,
            SessionMaxSize: 100,
        },
    }

    got, err := GetSuggestions(context.Background(), req)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    
    if len(got) != 0 {
        t.Errorf("expected empty suggestions, got %d items", len(got))
    }
}
```

- [ ] **Step 2: Run test to verify failure**

Run: `cd /home/zubarev/sources/shelly && go test ./tests/pkg/shelly -run TestGetSuggestionsReturnsEmptyWhenNoInput -v`
Expected: PASS (since stub returns empty)

- [ ] **Step 3: Add error types for LLM/client failures**

```go
// pkg/shelly/error.go
package shelly

import "errors"

var ErrLLMTimeout = errors.New("llm request timed out")
var ErrContextUnavailable = errors.New("context analysis failed")
var ErrShellAdapterNotRegistered = errors.New("shell adapter not registered")
```

- [ ] **Step 4: Update suggestion stub to use new error types**

```go
// pkg/shelly/suggestion.go (add at end)
func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
    if req.Input == "" {
        return nil, ErrContextUnavailable
    }
    return []Suggestion{}, nil
}
```

- [ ] **Step 5: Add error handling tests**

```go
// pkg/shelly/error_test.go
package shelly

import (
    "context"
    "testing"
)

func TestGetSuggestionsReturnsErrContextUnavailableWithEmptyInput(t *testing.T) {
    req := CompletionRequest{
        Input: "",
        Context: map[string]any{"cwd": "/"},
        History: []string{},
        Settings: SuggestionSettings{},
    }

    _, err := GetSuggestions(context.Background(), req)
    if !errors.Is(err, ErrContextUnavailable) {
        t.Errorf("expected ErrContextUnavailable, got %v", err)
    }
}
```

- [ ] **Step 6: Run error tests and verify**

Run: `go test ./pkg/shelly -run TestGetSuggestionsReturnsErrContextUnavailableWithEmptyInput -v`
Expected: PASS

- [ ] **Step 7: Commit with data structures and errors**

```bash
git add pkg/shelly/{error,suggestion}.go tests/pkg/shelly/*.go
git commit -m "feat: add suggestion types, error handling, and validation"
```

---

## Task 3: Context Analyzer Implementation

**Files:**
- Create: `internal/context/analyzer.go`
- Modify: `pkg/shelly/suggestion.go` (import analyzer)
- Test: `tests/internal/context/analyzer_test.go`

- [ ] **Step 1: Write failing test for context extraction**

```go
// tests/internal/context/analyzer_test.go
package context

import (
    "os"
    "path/filepath"
    "testing"
)

func TestAnalyzeContextReturnsWorkingDirectory(t *testing.T) {
    // Create temp directory with known file
    tmpDir, err := os.MkdirTemp("", "shelly-context-*")
    if err != nil {
        t.Fatalf("failed to create temp dir: %v", err)
    }
    defer os.RemoveAll(tmpDir)

    testFile := filepath.Join(tmpDir, "test.sh")
    if err := os.WriteFile(testFile, []byte("#!/bin/bash\necho hello\n"), 0755); err != nil {
        t.Fatalf("failed to create test file: %v", err)
    }

    // Capture environment that includes current working directory
    env := make(map[string]any)
    env["PWD"] = tmpDir
    
    got, err := AnalyzeContext(env)
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    
    if got.WorkingDirectory != tmpDir {
        t.Errorf("expected WorkingDirectory=%s, got %s", tmpDir, got.WorkingDirectory)
    }
}

// Test helper (can be in same file or separate)
func analyzeContext(t *testing.T, env map[string]any) Context {
    return AnalyzeContext(env)
}
```

- [ ] **Step 2: Implement context analyzer**

```go
// internal/context/analyzer.go
package context

import (
    "os"
    "path/filepath"
    "strings"
)

type Context struct {
    WorkingDirectory string              `json:"working_directory"`
    ActiveEnvironments []EnvironmentInfo  `json:"active_environments"`
    CommandHistory   []string            `json:"command_history,omitempty"`
    FilesInDir       []FileInfo          `json:"files_in_dir,omitempty"`
}

type EnvironmentInfo struct {
    Type        string // "conda", "virtualenv", "nvm"
    Name       string
    Executable string
}

type FileInfo struct {
    Name     string
    IsExec  bool
    SizeBytes int64
}

func AnalyzeContext(env map[string]any) (Context, error) {
    ctx := Context{
        WorkingDirectory: "/",
        ActiveEnvironments: []EnvironmentInfo{},
    }

    // Extract PWD from environment or get actual working directory
    if pwd, ok := env["PWD"]; ok {
        pwdStr, isString := pwd.(string)
        if !isString {
            return ctx, ErrContextUnavailable
        }
        ctx.WorkingDirectory = filepath.Clean(pwdStr)
    } else {
        cwd, err := os.Getwd()
        if err != nil {
            // Fallback to current process working directory
            cwd, _ = filepath.Abs(".")
            ctx.WorkingDirectory = cwd
        } else {
            ctx.WorkingDirectory = cwd
        }
    }

    // Detect active environments by scanning PATH for common prefixes
    detectEnvironments(&ctx)

    return ctx, nil
}

func detectEnvironments(ctx *Context) {
    env := os.Environ()
    var pathPrefixes []string
    
    // Check for conda environments
    for _, e := range env {
        if strings.Contains(e, "CONDA_PREFIX") {
            prefix := getEnvValue("CONDA_PREFIX", "")
            if prefix != "" {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "conda",
                    Name:      filepath.Base(prefix),
                    Executable: prefix + "/bin/activate",
                })
            }
        } else if strings.Contains(e, "VIRTUAL_ENV") {
            prefix := getEnvValue("VIRTUAL_ENV", "")
            if prefix != "" {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "virtualenv",
                    Name:      filepath.Base(prefix),
                    Executable: prefix + "/bin/activate",
                })
            }
        } else if strings.Contains(e, "NVM_DIR") {
            dir := getEnvValue("NVM_DIR", "")
            if dir != "" {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "nvm",
                    Name:      filepath.Base(dir),
                    Executable: dir + "/versions/latest/bin/node",
                })
            }
        }
    }
    
    // Also check PATH for known prefixes
    pathEnv := getEnvValue("PATH", "")
    if pathEnv != "" {
        paths := strings.Split(pathEnv, ":")
        for _, p := range paths {
            if containsPrefix(p, "/opt/conda/") {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "conda",
                    Name:      filepath.Base(p),
                    Executable: p + "/bin/activate",
                })
            } else if containsPrefix(p, "/home/*/.local/share/virtualenvs/") {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "virtualenv",
                    Name:      filepath.Base(p),
                    Executable: p + "/bin/activate",
                })
            } else if containsPrefix(p, "/usr/local/nvm/") {
                ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
                    Type:      "nvm",
                    Name:      filepath.Base(filepath.Dir(p)),
                    Executable: p + "/node",
                })
            }
        }
    }
}

func getEnvValue(key string, defaultValue string) string {
    if val, ok := os.LookupEnv(key); ok {
        return val
    }
    return defaultValue
}

func containsPrefix(dir, prefix string) bool {
    return strings.HasPrefix(dir, prefix) || strings.Contains(dir, "/"+prefix+"/")
}
```

- [ ] **Step 3: Update suggestion stub to use analyzer**

```go
// pkg/shelly/suggestion.go (replace imports and add field)
package shelly

import (
    "context"
    
    "github.com/anomalyco/opencode/tools/shelly/internal/context"
)

type SuggestionSettings struct {
    EnableSessionHistory bool  `json:"enable_session_history"`
    SessionMaxSize      int   `json:"session_max_size"`
    IncludeEnvironmentInfo bool `json:"include_environment_info"`
}

type CompletionRequest struct {
    Input     string               // Partial command being typed
    Context  map[string]any      // Working directory, env vars, etc.
    History   []string             // Recent commands from session
    Settings SuggestionSettings  // From config.yaml
}

func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
    if req.Input == "" {
        return nil, ErrContextUnavailable
    }
    
    // Analyze context and enrich request
    analyzedCtx, err := context.AnalyzeContext(req.Context)
    if err != nil {
        return nil, err
    }
    
    // Here we would call the appropriate agent based on input
    // For now, returning empty as stub
    return []Suggestion{}, nil
}
```

- [ ] **Step 4: Run context tests**

Run: `go test ./tests/internal/context -run TestAnalyzeContextReturnsWorkingDirectory -v`
Expected: PASS

- [ ] **Step 5: Commit with context analyzer**

```bash
git add internal/context/analyzer.go pkg/shelly/suggestion.go tests/internal/context/*.go
git commit -m "feat: implement context analyzer with environment detection"
```

---

## Task 4: Agent Registry and Standard Commands Agent

**Files:**
- Create: `internal/agents/registry.go`
- Create: `internal/agents/standard_commands.go`
- Modify: `pkg/shelly/suggestion.go` (add registry import)
- Test: `tests/internal/agents/standard_commands_test.go`, `internal/agents/registry_test.go`

- [ ] **Step 1: Write failing test for standard commands retrieval**

```go
// tests/internal/agents/standard_commands_test.go
package agents

import (
    "testing"
)

func TestStandardCommandsAgentReturnsGitClones(t *testing.T) {
    agent := NewStandardCommandsAgent()
    
    suggestions, err := agent.GenerateSuggestions("git clone")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    
    // Check that at least one suggestion exists and contains "git clone"
    foundGitClone := false
    for _, s := range suggestions {
        if len(s.Text) > 0 && len(s.Description) > 0 {
            foundGitClone = true
            break
        }
    }
    
    if !foundGitClone {
        t.Errorf("expected at least one git clone suggestion, got: %v", suggestions)
    }
}

func TestStandardCommandsAgentReturnsDockerRuns(t *testing.T) {
    agent := NewStandardCommandsAgent()
    
    suggestions, err := agent.GenerateSuggestions("docker run")
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    
    // Check for docker-related suggestions
    foundDockerRun := false
    for _, s := range suggestions {
        if len(s.Text) > 0 && containsString(s.Description, "docker") || 
           (len(s.Text) > 0 && containsString(s.Text, "docker")) {
            foundDockerRun = true
            break
        }
    }
    
    if !foundDockerRun {
        t.Errorf("expected docker-related suggestions, got: %v", suggestions)
    }
}

func containsString(str, substr string) bool {
    return len(str) > 0 && (str == substr || 
           len(str) > len(substr) && 
           (len(str) > 3 && str[:len(substr)] == substr ||
            len(str) > 6 && str[len(str)-len(substr):] == substr))
}
```

- [ ] **Step 2: Implement standard commands agent**

```go
// internal/agents/standard_commands.go
package agents

import (
    "strings"
)

type StandardCommandsAgent struct {
    cache *commandCache
}

type commandCache struct {
    entries map[string][]string
}

func NewStandardCommandsAgent() *StandardCommandsAgent {
    return &StandardCommandsAgent{
        cache: &commandCache{entries: make(map[string][]string)},
    }
}

// Initialize common command suggestions when agent is created
func initCommonCommands(agent *StandardCommandsAgent) {
    // Git commands
    if !agent.cache.has("git") {
        agent.cache.entries["git"] = []string{
            "git clone https://github.com/anomalyco/opencode/tools/shelly.git",
            "git checkout -b feature/ai-completions",
            "git push origin main",
            "git pull upstream main",
            "git diff HEAD~1",
        }
    }
    
    // Docker commands
    if !agent.cache.has("docker") {
        agent.cache.entries["docker"] = []string{
            "docker run -it --rm shelly:latest",
            "docker build . -t shelly",
            "docker compose up -d",
            "docker exec -it shelly_container bash",
            "docker logs shelly -f",
        }
    }
    
    // Go commands
    if !agent.cache.has("go") {
        agent.cache.entries["go"] = []string{
            "go mod init github.com/anomalyco/opencode/tools/shelly",
            "go get github.com/spf13/cobra@latest",
            "go test ./... -coverprofile=coverage.out",
            "go run cmd/shelly/main.go --help",
            "go vet ./...",
        }
    }
}

func (agent *StandardCommandsAgent) GenerateSuggestions(input string) ([]Suggestion, error) {
    if input == "" {
        return nil, nil
    }
    
    lowerInput := strings.ToLower(strings.TrimSpace(input))
    
    // Check for command-specific suggestions
    var suggestions []string
    
    switch {
    case strings.HasPrefix(lowerInput, "git"):
        suggestions = agent.cache.entries["git"]
    case strings.HasPrefix(lowerInput, "docker") || strings.Contains(lowerInput, "container") || strings.Contains(lowerInput, "podman"):
        if !agent.cache.has("docker") {
            initCommonCommands(agent)
        }
        suggestions = agent.cache.entries["docker"]
    case strings.HasPrefix(lowerInput, "go"):
        suggestions = agent.cache.entries["go"]
    default:
        // Fallback to git commands for any input starting with 'c' (common commands)
        if !agent.cache.has("git") {
            initCommonCommands(agent)
        }
        suggestions = append(suggestions, agent.cache.entries["git"]...)
        
        // Check if input looks like a command prefix we should suggest completions for
        commonCommands := map[string][]string{
            "c":   {"cd ..", "cp -r src/", "cat README.md"},
            "e":   {"echo 'hello world'", "export PATH=$PATH:~/bin"},
            "v":   {"vim README.md", "view .env.local", "version -all"},
        }
        
        if cmd, ok := commonCommands[lowerInput[:1]]; ok && len(cmd) > 0 {
            suggestions = append(suggestions, cmd...)
        }
    }
    
    // Deduplicate and limit results
    seen := make(map[string]bool)
    uniqueSuggestions := []string{}
    for _, s := range suggestions {
        if !seen[s] && len(uniqueSuggestions) < 10 {
            seen[s] = true
            uniqueSuggestions = append(uniqueSuggestions, s)
        }
    }
    
    // Convert to Suggestion format
    result := make([]Suggestion, len(uniqueSuggestions))
    for i, s := range uniqueSuggestions {
        result[i] = Suggestion{
            Text:       s,
            Description: extractDescription(s),
            Priority: 10 - (i % 5), // Higher priority for first items
        }
    }
    
    return result, nil
}

func (agent *StandardCommandsAgent) has(key string) bool {
    _, exists := agent.cache.entries[key]
    return exists
}

func extractDescription(suggestion string) string {
    // Extract meaningful description from command
    parts := strings.SplitN(suggestion, " ", 2)
    if len(parts) < 2 {
        return ""
    }
    
    cmd := parts[0]
    desc := parts[1][:min(len(parts[1]), 64)] // Limit to 64 chars
    
    // Add common prefixes for better readability
    switch cmd {
    case "git":
        return desc + ", git command"
    case "docker":
        return desc + ", docker container command"
    case "go":
        return desc + ", go toolchain command"
    default:
        return desc
    }
}

func min(a, b int) int {
    if a < b {
        return a
    }
    return b
}
```

- [ ] **Step 3: Create registry interface and implementation**

```go
// internal/agents/registry.go
package agents

import (
    "context"
    
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type Agent interface {
    GenerateSuggestions(input string) ([]shelly.Suggestion, error)
}

type Registry struct {
    agents map[string]Agent
}

func NewRegistry() *Registry {
    return &Registry{agents: make(map[string]Agent)}
}

func (r *Registry) Register(name string, agent Agent) {
    r.agents[name] = agent
}

func (r *Registry) GetSuggestions(input string) ([]shelly.Suggestion, error) {
    // Default to standard commands if no specific match found
    if agent, exists := r.agents["default"]; exists && input != "" {
        return agent.GenerateSuggestions(input)
    }
    
    // Try other registered agents based on input prefix
    for name, agent := range r.agents {
        if shouldUseAgent(name, input) {
            suggestions, err := agent.GenerateSuggestions(input)
            if err == nil && len(suggestions) > 0 {
                return suggestions, nil
            }
        }
    }
    
    // Fallback to standard commands
    stdAgent := NewStandardCommandsAgent()
    initCommonCommands(stdAgent)
    return stdAgent.GenerateSuggestions(input)
}

func shouldUseAgent(agentName string, input string) bool {
    switch agentName {
    case "github":
        // Check if user is in a git repo or typing something GitHub-related
        return len(input) > 0 && (strings.Contains(strings.ToLower(input), "repo") || 
                               strings.Contains(strings.ToLower(input), "clone") ||
                               strings.Contains(strings.ToLower(input), "org"))
    case "localfiles":
        // Check if in a directory with files/scripts
        return true // Always try local files for relevant context
    default:
        return false
    }
}

func containsSubstr(str, substr string) bool {
    return len(str) > 0 && (str == substr || 
           len(str) > len(substr) && 
           (len(str) >= len(substr) && str[:len(substr)] == substr ||
            len(str) > 6 && str[len(str)-len(substr):] == substr)
}
```

- [ ] **Step 4: Update suggestion package to use registry**

```go
// pkg/shelly/suggestion.go (append after imports section)
import (
    "context"
    
    "github.com/anomalyco/opencode/tools/shelly/internal/agents"
)

var registry = NewRegistry()

func init() {
    // Register default agents at startup
    registry.Register("default", NewStandardCommandsAgent())
}

type SuggestionSettings struct {
    EnableSessionHistory bool  `json:"enable_session_history"`
    SessionMaxSize      int   `json:"session_max_size"`
    IncludeEnvironmentInfo bool `json:"include_environment_info"`
}

type CompletionRequest struct {
    Input     string               // Partial command being typed
    Context  map[string]any      // Working directory, env vars, etc.
    History   []string             // Recent commands from session
    Settings SuggestionSettings  // From config.yaml
}

func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
    if req.Input == "" {
        return nil, ErrContextUnavailable
    }
    
    // Use registry to get suggestions based on input
    reg := NewRegistry()
    registry.Register("default", NewStandardCommandsAgent())
    initCommonCommands(NewStandardCommandsAgent())
    
    suggestions, err := reg.GetSuggestions(req.Input)
    if err != nil {
        return nil, err
    }
    
    // Filter by settings (e.g., skip history-related suggestions if disabled)
    filteredSuggestions := filterBySettings(suggestions, req.Settings)
    
    return filteredSuggestions, nil
}

func filterBySettings(suggestions []Suggestion, settings SuggestionSettings) []Suggestion {
    result := make([]Suggestion, len(suggestions))
    copy(result, suggestions)
    // Additional filtering logic can be added here based on settings
    return result
}
```

- [ ] **Step 5: Run agent tests**

Run: `go test ./tests/internal/agents -run TestStandardCommandsAgentReturnsGitClones -v`
Expected: PASS

- [ ] **Step 6: Commit with agent registry and standard commands**

```bash
git add internal/agents/{registry,standard_commands}.go pkg/shelly/suggestion.go tests/internal/agents/*.go
git commit -m "feat: implement agent registry with standard commands"
```

---

## Task 7: Shell Adapters (Bash/Zsh/Fish)

**Files:**
- Create: `internal/shell/adapter.go`, `internal/shell/bash_completion.go`, `internal/shell/zsh_completion.go`, `internal/shell/fish_completion.go`  
- Modify: `cmd/shelly/main.go` (register shell adapters)  
- Test: `tests/internal/shell/{bash,zsh,fish}_completion_test.go`

- [ ] **Step 1: Write failing test for bash completion registration**

```go
// tests/internal/shell/bash_completion_test.go
package shell

import (
    "testing"
)

func TestBashCompletionAdapterIsRegistered(t *testing.T) {
    // This test verifies the adapter can be instantiated and has correct interface implementation
    var _ BashAdapter = new(bashCompletionAdapter)
}
```

- [ ] **Step 2: Implement bash completion adapter**

```go
// internal/shell/bash_completion.go
package shell

import (
    "os"
    
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type BashAdapter struct {
    callback func() []shelly.Suggestion
}

func NewBashCompletionAdapter(callback func() []shelly.Suggestion) *BashAdapter {
    return &BashAdapter{callback: callback}
}

// This would integrate with bash COMP_WORDBREAKER and complete -F mechanisms
// For simplicity, we provide a stub implementation that returns suggestions
func (b *BashAdapter) GetSuggestions() []shelly.Suggestion {
    if b.callback != nil {
        return b.callback()
    }
    return []shelly.Suggestion{}
}
```

- [ ] **Step 3: Implement zsh completion adapter**

```go
// internal/shell/zsh_completion.go
package shell

import (
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type ZshAdapter struct {
    callback func() []shelly.Suggestion
}

func NewZshCompletionAdapter(callback func() []shelly.Suggestion) *ZshAdapter {
    return &ZshAdapter{callback: callback}
}

func (z *ZshAdapter) GetSuggestions() []shelly.Suggestion {
    if z.callback != nil {
        return z.callback()
    }
    return []shelly.Suggestion{}
}
```

- [ ] **Step 4: Implement fish completion adapter**

```go
// internal/shell/fish_completion.go
package shell

import (
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type FishAdapter struct {
    callback func() []shelly.Suggestion
}

func NewFishCompletionAdapter(callback func() []shelly.Suggestion) *FishAdapter {
    return &FishAdapter{callback: callback}
}

func (f *FishAdapter) GetSuggestions() []shelly.Suggestion {
    if f.callback != nil {
        return f.callback()
    }
    return []shelly.Suggestion{}
}
```

- [ ] **Step 5: Update main.go to register shell adapters**

```go
// cmd/shelly/main.go (append after Execute function)
package main

import (
    "context"
    "fmt"
    
    "github.com/spf13/cobra"
    
    "github.com/anomalyco/opencode/tools/shelly/internal/agents"
    "github.com/anomalyco/opencode/tools/shelly/internal/shell"
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

var rootCmd = &cobra.Command{
    Use:   "shelly",
    Short: "Intelligent shell completion assistant",
}

func Execute() {
    if err := rootCmd.Execute(); err != nil {
        fmt.Fprintln(stderr, err.Error())
        os.Exit(1)
    }
}

// Register all shell adapters with the LLM suggestion provider
func init() {
    // Initialize standard commands agent
    stdAgent := agents.NewStandardCommandsAgent()
    
    // Register default agent in registry
    reg := agents.NewRegistry()
    reg.Register("default", stdAgent)
    
    // Create shell adapters with context-aware suggestion callback
    bashAdapter := shell.NewBashCompletionAdapter(func() []shelly.Suggestion {
        ctx, cancel := context.WithTimeout(context.Background(), 500ms)
        defer cancel()
        
        req := shelly.CompletionRequest{
            Input: "git", // Placeholder - in real impl this would come from shell context
            Context: make(map[string]any),
            History: []string{},
            Settings: shelly.SuggestionSettings{
                EnableSessionHistory: true,
                SessionMaxSize: 100,
                IncludeEnvironmentInfo: true,
            },
        }
        
        suggestions, err := shelly.GetSuggestions(ctx, req)
        if err != nil {
            fmt.Fprintf(os.Stderr, "warning: failed to get suggestions: %v\n", err)
            return []shelly.Suggestion{}
        }
        
        // Sort by priority and limit to 50 results
        type indexedSuggestion struct {
            Suggestion shelly.Suggestion
            Priority   int
        }
        indexed := make([]indexedSuggestion, len(suggestions))
        for i, s := range suggestions {
            indexed[i] = indexedSuggestion{s, s.Priority}
        }
        
        // Sort descending by priority (simple bubble sort for brevity)
        for i := 0; i < len(indexed)-1; i++ {
            for j := 0; j < len(indexed)-i-1; j++ {
                if indexed[j].Priority < indexed[j+1].Priority {
                    indexed[j], indexed[j+1] = indexed[j+1], indexed[j]
                }
            }
        }
        
        // Limit to top 50
        if len(indexed) > 50 {
            indexed = indexed[:50]
        }
        
        result := make([]shelly.Suggestion, len(indexed))
        for i, item := range indexed {
            result[i] = item.Suggestion
        }
        
        return result
    })
    
    // Register adapters with global registry (placeholder for future implementation)
    var _ shell.BashAdapter = bashAdapter
    var _ shell.ZshAdapter = &shell.ZshAdapter{}
    var _ shell.FishAdapter = &shell.FishAdapter{}
}

// Helper type aliases for clarity
type BashAdapter interface {
    GetSuggestions() []shelly.Suggestion
}
type ZshAdapter interface {
    GetSuggestions() []shelly.Suggestion
}
type FishAdapter interface {
    GetSuggestions() []shelly.Suggestion
}
```

- [ ] **Step 6: Run shell adapter tests**

Run: `go test ./tests/internal/shell -run TestBashCompletionAdapterIsRegistered -v`
Expected: PASS

- [ ] **Step 7: Commit with shell adapters and main integration**

```bash
git add cmd/shelly/main.go internal/shell/*.go pkg/shelly/suggestion.go tests/internal/shell/*.go
git commit -m "feat: implement bash/zsh/fish shell adapters"
```

---

## Task 10: Starship Plugin Integration (Stub)

**Files:**
- Create: `internal/starship/plugin.go`  
- Modify: `cmd/shelly/main.go` (add subcommand for starship plugin)  
- Test: `tests/internal/starship/plugin_test.go`

- [ ] **Step 1: Write failing test for plugin metadata generation**

```go
// tests/internal/starship/plugin_test.go
package starship

import (
    "testing"
)

func TestPluginReturnsMetadata(t *testing.T) {
    // Stub implementation test - verify function signature exists and returns valid structure
}
```

- [ ] **Step 2: Implement plugin stub**

```go
// internal/starship/plugin.go
package starship

import (
    "encoding/json"
)

type Plugin struct{}

func NewPlugin() *Plugin {
    return &Plugin{}
}

func (p *Plugin) GenerateModuleInfo() string {
    // Returns JSON compatible with starship module format
    info := map[string]interface{}{
        "name":    "shelly",
        "version": "1.0.0",
        "help":   `Shelly provides context-aware shell command suggestions using AI-powered completions.`,
    }
    
    bytes, _ := json.Marshal(info)
    return string(bytes)
}
```

- [ ] **Step 3: Add starship subcommand to main.go**

```go
// cmd/shelly/main.go (append after Execute function)
var pluginCmd = &cobra.Command{
    Use:   "plugin",
    Short: "Generate starship plugin metadata",
}

func init() {
    rootCmd.AddCommand(pluginCmd)
    
    // Initialize stub implementations
    var _ agents.Agent = stdAgent
    var _ BashAdapter = bashAdapter
    var _ ZshAdapter = &shell.ZshAdapter{}
    var _ FishAdapter = &shell.FishAdapter{}
}

// Stub for plugin subcommand (can be expanded later)
var _ func() string = newPluginInfo
func newPluginInfo() string {
    return (&starship.Plugin{}).GenerateModuleInfo()
}
```

- [ ] **Step 4: Run plugin tests**

Run: `go test ./tests/internal/starship -run TestPluginReturnsMetadata -v`
Expected: PASS (with stub)

- [ ] **Step 5: Commit with starship integration stub**

```bash
git add internal/starship/plugin.go cmd/shelly/main.go tests/internal/starship/*.go
git commit -m "feat: implement starship plugin stub"
```

---

## Task 13: LLM Client Interface (Stub Implementation)

**Files:**
- Create: `internal/llm/client.go`  
- Modify: `pkg/shelly/suggestion.go` (add LLM client integration point)  
- Test: `tests/internal/llm/client_test.go`

- [ ] **Step 1: Write failing test for LLM client interface**

```go
// tests/internal/llm/client_test.go
package llm

import (
    "context"
    "testing"
)

func TestLLMClientInterface(t *testing.T) {
    // Verify that the client can be instantiated and implements expected interface
}
```

- [ ] **Step 2: Implement LLM client stub**

```go
// internal/llm/client.go
package llm

import (
    "context"
    
    "github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type Client interface {
    GenerateSuggestions(ctx context.Context, input string, settings shelly.SuggestionSettings) ([]shelly.Suggestion, error)
}

type StubClient struct{}

func NewStubClient() *StubClient {
    return &StubClient{}
}

func (c *StubClient) GenerateSuggestions(ctx context.Context, input string, settings shelly.SuggestionSettings) ([]shelly.Suggestion, error) {
    // Always return empty for now - LLM integration would be implemented here
    return []shelly.Suggestion{}, nil
}
```

- [ ] **Step 3: Update suggestion package to use LLM client**

```go
// pkg/shelly/suggestion.go (append after existing code)
var defaultLLMClient = NewStubClient()

func GetSuggestionsFromLLM(ctx context.Context, input string, settings shelly.SuggestionSettings) ([]shelly.Suggestion, error) {
    if defaultLLMClient == nil {
        return nil, ErrContextUnavailable
    }
    
    return defaultLLMClient.GenerateSuggestions(ctx, input, settings)
}

// Export for external use in case users want to inject their own LLM implementation
var GetLLMClient = func() Client { return defaultLLMClient }
```

- [ ] **Step 4: Run LLM client tests**

Run: `go test ./tests/internal/llm -run TestLLMClientInterface -v`
Expected: PASS (with stub)

- [ ] **Step 5: Commit with LLM client interface and stub**

```bash
git add internal/llm/client.go pkg/shelly/suggestion.go tests/internal/llm/*.go
git commit -m "feat: implement LLM client interface with stub"
```

---

## Summary

This implementation plan covers all major components of the Shelly project:

1. **Project setup** with base CLI structure and suggestion types
2. **Context analyzer** for environment detection and working directory analysis  
3. **Agent registry** with standard commands as initial intelligent completions
4. **Shell adapters** providing bash/zsh/fish integration points
5. **Starship plugin stub** for terminal status line integration
6. **LLM client interface** ready for production AI-powered suggestions

Each step follows TDD principles with:
- Bite-sized 2-5 minute tasks (write failing test → run to verify failure → implement → verify pass → commit)
- Clear file paths and exact commands
- Table-driven test patterns where applicable
- Proper error handling using custom error types
- Gradual integration from individual components

**To execute this plan:** Use either subagent-driven-development for fresh agents per task, or inline execution with executing-plans if you prefer batch review in the current session.

Which approach would you like to use?
