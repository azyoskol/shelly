# Shally Framework

**Shally** is a modular Rust platform for integrating AI-powered prompts into your command line. The architecture supports a flexible plugin system, seamless integration with Zsh, Fish, Bash, and Starship shells, and MVP AI integration via external APIs or local models.

## Key Features

- **Plugin Architecture**: Clean separation of the core framework from plugins via the `ShellPlugin` trait
- **Shell Integration**: Native support for Zsh (precmd/preexec), Fish (prompt/command_not_found), Bash (PROMPT_COMMAND/DEBUG trap), and Starship via `SHELLAI_*` environment variables
- **AI-Powered Prompts**: MVP AI module supporting external APIs (OpenAI-compatible) and local models
- **CLI-First Design**: Comprehensive CLI flags for quick testing, installation, and configuration
- **Modular Codebase**: Well-organized source code with clear separation of concerns

## Project Structure

```
src/
├── main.rs          # CLI entry point and command orchestration
├── lib.rs           # Module exports and public API
├── core/            # Core types and traits
│   ├── types.rs     # PluginConfig, ShellContext
│   └── plugin.rs    # ShellPlugin trait definition
├── cli/             # CLI resolution and dispatch logic
├── commands/        # Command handlers
│   ├── install.rs   # Zsh/Fish installation snippets
│   ├── context.rs   # Context export for Starship
│   ├── ai_config.rs # AI API configuration
│   └── history.rs   # History management (recent, search, clear)
├── hooks/           # Shell hook implementations
│   ├── zsh.rs       # Zsh precmd/preexec hooks
│   ├── fish.rs      # Fish prompt/command_not_found hooks
│   └── bash.rs      # Bash precmd/preexec hooks
├── ai/              # AI integration layer
├── config/          # Configuration management
├── history/         # History storage and retrieval
├── plugin/          # Plugin system implementation
├── starship/        # Starship shell integration
├── zsh/             # Zsh-specific utilities
├── fish/            # Fish-specific utilities
└── bash/            # Bash-specific utilities
```

## Installation

### Requirements

- Rust toolchain (stable)
- Cargo package manager

### Build & Run

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd shally
   ```

2. **Build the project**:
   ```bash
   cargo build
   ```

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Run the binary**:
   
   The compiled binary is named `shally`.
   
   - **Quick start (Zsh)**:
     ```bash
     cargo run -- --install
     # Prints a .zshrc snippet to add to your shell config
     ```
   
   - **Fish shell config**:
     ```bash
     cargo run -- --fish-install
     # Prints Fish shell configuration snippet
     ```
   
   - **Bash shell config**:
     ```bash
     cargo run -- --bash-install
     # Prints Bash shell configuration snippet
     ```
   
   - **Configure AI API**:
     ```bash
     cargo run -- --ai-config https://api.openai.com/v1/chat/completions <api_key> [model]
     # Example: cargo run -- --ai-config https://api.openai.com/v1/chat/completions sk-KEY gpt-4
     ```

## CLI Commands

| Command | Description |
|---------|-------------|
| `--install` | Generate Zsh installation snippet |
| `--fish-install` | Generate Fish shell installation snippet |
| `--bash-install` | Generate Bash shell installation snippet |
| `--export-context` | Export current context for Starship integration |
| `--ai-config <url> <key> [model]` | Configure AI API endpoint and credentials |
| `--history [recent\|search\|clear]` | Manage command history |
| `--help` | Display help information |

### Test Mode for Local Models

For testing without real API calls, use test mode:

```bash
export SHELLAI_TEST_MODE=1
cargo test
```

Local models do not require an API key. Support for local model CLI configuration may evolve in future releases.

## Output Examples

### Zsh Installation Snippet

```bash
$ cargo run -- --install
# Shally Framework - Zsh Integration
# Add the following to your ~/.zshrc:
# ... (hook definitions)
```

### Fish Installation Snippet

```bash
$ cargo run -- --fish-install
# Shally Framework - Fish Integration
# Add the following to your Fish config:
# ... (hook definitions)
```

### Bash Installation Snippet

```bash
$ cargo run -- --bash-install
# Shally Framework - Bash Integration
# Add the following to your ~/.bashrc:
# ... (hook definitions including PROMPT_COMMAND and DEBUG trap)
```

### Starship Context Export

```bash
$ cargo run -- --export-context
SHELLAI_CURRENT_DIR=/home/user
SHELLAI_PWD=/home/user
```

### AI Configuration

```bash
$ cargo run -- --ai-config https://api.openai.com/v1/chat/completions sk-KEY gpt-4
Command: ls -la
Explanation: Lists all files including hidden ones in long format
Confidence: 0.85
```

### Test Mode

```bash
$ export SHELLAI_TEST_MODE=1
$ cargo test
# Runs tests with mock AI responses
```

## Architecture Overview

- **`src/lib.rs`**: Module exports and public API definitions
- **`src/main.rs`**: CLI argument parsing and command dispatch
- **`src/core/`**: Fundamental types (`PluginConfig`, `ShellContext`) and the `ShellPlugin` trait
- **`src/cli/`**: Configuration resolution and command routing logic
- **`src/commands/`**: Individual command handlers for each CLI operation
- **`src/hooks/`**: Shell-specific hook implementations for Zsh, Fish, and Bash
- **`src/plugin/`**: Plugin lifecycle management and `MockPlugin` for testing
- **`src/zsh/`, `src/fish/`, & `src/bash/`**: Shell-specific utilities and snippet generators
- **`src/starship/`**: Environment variable export for Starship integration
- **`src/ai/`**: AI integration layer with API client and local model support
- **`src/config/`**: Configuration file parsing and management
- **`src/history/`**: Command history storage and retrieval

## Testing

All tests are run via `cargo test`. The test suite includes:

- Unit tests for core functionality
- Integration tests for API calls
- Mock-based tests for plugin system
- Test mode support for local development without API calls

## Contributing

Contributions are welcome! Please follow these guidelines:

1. **Open an Issue**: Describe the feature or bug you'd like to address
2. **Fork the Repository**: Create your feature branch
3. **Implement Changes**: Ensure code follows existing patterns
4. **Add Tests**: Maintain or improve test coverage
5. **Submit a PR**: Include a clear description of changes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Enhanced local model support
- [ ] Additional shell integrations (PowerShell)
- [ ] Plugin marketplace/discovery
- [ ] Advanced AI prompt templates
- [ ] Performance optimizations
- [ ] Documentation improvements
