-# Shally Framework (Shelly)

Shally Framework is a modular Rust platform for integrating AI-powered prompts into the command line. The architecture supports a plugin system, integration with Zsh, Fish, and Starship, and MVP AI integration via an external API or local models.

## Key Ideas
- Separation of the core framework from plugins via the ShellPlugin trait
- Zsh (precmd/preexec), Fish (prompt/command_not_found) integration and exporting context to Starship via SHELLAI_* environment variables
- MVP AI prompts module with external API (OpenAI-compatible) and local models
- CLI flags for quick testing and demonstration

## Highlights
- Core Framework: plugin lifecycle and context
- Plugin System: MockPlugin for testing and a real plugin architecture
- Zsh, Fish, Starship integrations: example hooks, snippet generators, and context export
- AI API Hook: sending requests to the API and displaying prompts
- Local model support (optional via test mode)

## Installation
Requirements: Rust toolchain (stable) and cargo.

1. Clone the repository (path will vary):
   git clone <repository>
   cd shelly

2. Build the project:
   cargo build

3. Run tests:
   cargo test

4. Run the binary:
   - By default, the binary named `shally` is produced.
   - Quick start (Zsh):
     cargo run -- --install
     # prints a .zshrc snippet
   - Fish config:
     cargo run -- --fish-install
   - AI API configuration:
     cargo run -- --ai-config https://api.openai.com/v1/chat/completions <api_key> [model]
     # example: gpt-4

5. Local models and test mode
   - Local models do not require an API key.
   - For testing without real API calls: 
     export SHELLAI_TEST_MODE=1
     cargo test
   - Local-mode CLI support may evolve in future releases.

## CLI Commands
- Install Zsh snippet: cargo run -- --install
- Install Fish config: cargo run -- --fish-install
- Export context for Starship: cargo run -- --export-context
- AI config via API: cargo run -- --ai-config https://api.openai.com/v1/chat/completions <api_key> gpt-4
- Test mode for local models: export SHELLAI_TEST_MODE=1; cargo test
- Local mode example (planned): see test mode

## Output Examples
- Zsh snippet output:
```
$ cargo run -- --install
# Shally Framework - Zsh Integration
# <snippet output>...
```
- Fish snippet output:
```
$ cargo run -- --fish-install
# Shally Framework - Fish Integration
# <snippet output>...
```
- Starship context export:
```
$ cargo run -- --export-context
SHELLAI_CURRENT_DIR=/home/user
SHELLAI_PWD=/home/user
```
- AI config via API:
```
$ cargo run -- --ai-config https://api.openai.com/v1/chat/completions sk-KEY gpt-4
Command: ls -la
Explanation: None
Confidence: 0.85
```
- Test mode for local models:
```
export SHELLAI_TEST_MODE=1
cargo test
```

## Testing
- All tests run via cargo test. The repository includes tests for local modes and API integration.

## Architecture (brief)
- src/lib.rs: module exports and API
- src/main.rs: CLI for demonstration
- src/plugin/: plugin system and MockPlugin
- src/zsh/: Zsh hooks and snippet generators
- src/fish/: Fish hooks and configs
- src/starship/: export context to SHELLAI_*
- src/ai/: AI integration (AiIntegration, AiSuggestion) and API/Local model calls

## Contributing
- Contributions are welcome. Please open issues/PRs describing changes and test coverage.

## License
- License not specified in README; please refer to repository licenses (MIT/Apache-2.0, etc.).
