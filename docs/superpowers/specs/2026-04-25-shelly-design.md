# Shelly - Intelligent Shell Completion Assistant

## 1. Overview

Shelly is a production-ready CLI tool written in Go that provides intelligent, context-aware command completions for bash, zsh, and fish shells with seamless starship integration.

### Key Features
- **AI-Powered Completions**: Leverages LLMs (OpenAI, Anthropic, HuggingFace) or local models (Ollama, LMStudio)
- **Context-Aware Suggestions**: Analyzes current command, working directory, environment, and session history
- **Multiple Providers Support**: Unified interface for both cloud-based and local LLM deployments
- **Configurable Context/Session Settings**: All settings via `config.yaml` for easy customization
- **Graceful Degradation**: Falls back to cached results or standard completions when LLM unavailable

## 2. Architecture

```
┌─────────────┐     ┌───────────┐     ┌───────────┐
│  LLM Client │◀──▶│ Agent Router │◀───▶│Agent Registry│
└─────────────┘     └───────────┘     └───────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
    ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
    │  Bash/Zsh   │   │   Fish      │   │  Starship   │
    │  Completion │   │  Completion │   │  Plugin     │
    └─────────────┘   └─────────────┘   └─────────────┘
```

### Core Components

#### 2.1 Agent Registry
- **StandardCommandsAgent**: Basic shell commands with smart prioritization (e.g., common git, docker commands)
- **GitHubProjectsAgent**: Repo search, git clone/checkout suggestions from GitHub/GitLab APIs
- **LocalFilesAgent**: Local file and script completions based on working directory context
- **LLMCommandGeneratorAgent**: AI-generated command suggestions with natural language understanding

#### 2.2 Context Analyzer
Analyzes current execution context to select relevant agents:
- Current partial command being typed
- Working directory and project structure
- Active shell environments (conda/virtualenv)
- Installed packages and dependencies
- Command history for session continuity
- Session context from configurable settings

#### 2.3 LLM Integration Layer
Unified interface supporting multiple providers:
- **Cloud Providers**: OpenAI, Anthropic (Claude), HuggingFace Inference API
- **Local Providers**: Ollama, LMStudio (via HTTP/gRPC)
- Rate limiting and request queuing
- Context/session management from `config.yaml`

#### 2.4 Shell Adapter Interface
Abstraction layer for different shell completion mechanisms:
- Bash: `COMP_WORDBREAKER`, `complete -F` integration
- Zsh: `compinit -C` with zsh-completions framework
- Fish: `fish_complete` functions with callback support

## 3. Data Flow

### Autocompletion Process
1. User types partial command (e.g., `git cl`)
2. ContextAnalyzer determines: current directory, active environments, command history, and session context from configurable settings
3. AgentRouter selects relevant agents based on context and queries LLM for suggestions
4. Results filtered by priority and deduplicated
5. Suggestions passed to shell adapter for display via shell's completion system

### Starship Integration Process
1. User invokes starship module/prompt command
2. Module requests current status/suggestions from LLM with configurable session context
3. Result formatted and returned to starship for display in terminal status line

## 4. Error Handling Strategy

- **Graceful Degradation**: If primary LLM unavailable, fall back to cached results or standard completions
- **Rate Limiting**: Implement exponential backoff with maximum retry attempts per provider
- **Fallback Chain**: Failover priority: OpenAI → Anthropic → HuggingFace → Ollama → LMStudio → Cached → Standard

## 5. Testing Strategy

### Unit Tests
- Each agent tested independently for suggestion accuracy
- Coverage target: >80% code coverage per package

### Integration Tests
- End-to-end tests with different shell adapters and LLM providers (OpenAI, Anthropic, HuggingFace, Ollama, LMStudio)
- Mocked LLM responses for deterministic testing

### Performance Tests
- Autocompletion response time < 100ms for standard commands (from cache or local data)
- LLM queries: <2 seconds P95 latency target

## 6. Configuration Schema

```yaml
# config.yaml example
telemetry:
  enabled: true
  api_url: https://telemetry.shelly.ai/v1/event

llm:
  provider: "openai"  # Options: openai, anthropic, huggingface, ollama, lmstudio
  model: "gpt-4o"
  max_tokens: 2048
  temperature: 0.7
  timeout_seconds: 30

context:
  enable_session_history: true  # Store previous interactions for better suggestions
  session_max_size: 100        # Maximum entries to retain in session history
  enable_project_detection: true  # Auto-detect project from working directory
  include_environment_info: true  # Include active conda/virtualenv info

rate_limiting:
  requests_per_second: 10
  burst_size: 20
fallback:
  use_cached_results: true
  cache_ttl_seconds: 3600
```

## 7. Project Structure (Go Conventions)
```
shelly/
├── cmd/
│   └── shelly/
│       └── main.go               # CLI entry point with Cobra commands
├── internal/                      # Private implementation details
│   ├── agents/                    # Agent implementations and registry
│   │   ├── registry.go            # Agent registration and discovery
│   │   ├── standard_commands.go  # Default shell command completions
│   │   ├── github_projects.go    # GitHub/GitLab repo completion agent
│   │   ├── local_files.go        # Local file system completion agent
│   │   └── llm_command_generator.go # LLM-powered command generation
│   ├── context/                   # Context analysis and session management
│   │   └── analyzer.go           # Current execution context extraction
│   ├── shell/                     # Shell-specific adapters
│   │   ├── adapter.go            # Interface for all shell adapters
│   │   ├── bash_completion.go    # Bash completion implementation
│   │   ├── zsh_completion.go     # Zsh completion implementation  
│   │   └── fish_completion.go    # Fish completion implementation
│   └── llm/                       # LLM integration layer
│       └── client.go             # Provider-agnostic LLM client interface
├── pkg/                           # Public packages (if any)
│   └── shelly/                    # Reusable components for other projects
│       ├── completion.go         # Completion data structures and types
│       └── suggestion.go         # Suggestion formatting utilities
├── config.yaml                     # Default configuration template
├── go.mod                          # Go module file
├── go.sum                          # Module checksums
└── README.md                       # Project documentation
```

## 8. Success Criteria

1. **Seamless Integration**: Zero friction setup for bash/zsh/fish completions with autocomplete support
2. **Context Awareness**: Accurate and relevant command suggestions based on current context
3. **Performance Targets**:
   - < 100ms latency for standard commands (cached responses)
   - LLM queries: <2 seconds P95 latency
4. **Backward Compatibility**: No breaking changes to existing shell workflows
5. **Configurability**: All context and session settings via `config.yaml` with sensible defaults
6. **Provider Flexibility**: Support for both cloud-based (OpenAI, Anthropic) and local LLMs (Ollama, LMStudio)

## 9. Security Considerations

- Never store API keys in repository; use environment variables or secure storage
- Validate all external inputs from shell adapters to prevent injection attacks
- Implement TLS for all external HTTP connections including Ollama/LMStudio
- Sanitize and escape all user input before displaying in terminal output

## 10. Versioning

Follow Semantic Versioning (SemVer):
- MAJOR: Breaking changes to CLI interface or shell adapter behavior
- MINOR: New features, backwards-compatible improvements
- PATCH: Bug fixes and non-breaking changes

## 11. Contribution Guidelines

See `CONTRIBUTING.md` for coding standards, PR process, and code review guidelines.

## 12. License

MIT License - see `LICENSE` file for details.