# ShellAI Framework Development Plan: Phase 1 (MVP) - FINAL BUILD PLAN

## Overview
The objective of Phase 1 is to establish a stable, modular shell framework foundation built in Rust. This phase shifts us from research/planning mode into the initial implementation stage. We will focus on achieving superior user experience through advanced context awareness and AI-driven command suggestions, ensuring compatibility with Zsh, Fish, and Starship right from the start.

**Language:** Rust
**Architecture:** Plugin/Framework System (Modular and Extensible).

### 🚀 Phase 1 Goal: Context & Completion MVP
*   **Key Deliverables:** Functional core framework, working configuration loader, robust history search, and an operational AI API hook for command suggestions.

### 🧱 Technical Components
1.  **Core Framework (`shell-core`):** The main runner responsible for executing the plugin lifecycle.
2.  **Plugin Manager (`plugin-manager`):** Implements the `ShellPlugin` trait/interface, ensuring that all features are modular and decoupled.
3.  **Multi-Shell Compatibility Layer:**
     *   **Zsh Hooks:** Direct integration using Zsh's native hook system for maximum state access (e.g., `precmd`).
     *   **Fish Shell:** Integration using Fish's event system (`fish_prompt`, `fish_command_not_found`).
     *   **Starship Integration:** Context Injection via `SHELLAI_*` environment variables that Starship can read and display in its prompt.
4.  **AI Integration Module (`ai-integration`):** Command Completion & Suggestions powered by external LLM providers.

### 🤖 AI Integration Module (`ai-integration`): The Primary Focus
*   **Feature:** Command Completion & Suggestions (MVP AI).
*   **Process Flow:** Input $\to$ Context Capture (Current command + full history) $\to$ API Call $\to$ Suggestion Display.
*   **Supported Providers:** OpenAI-compatible APIs (GPT-3.5, GPT-4, etc.)
*   **CLI Integration:** `--ai-config <endpoint> <api_key> [model]`

### 🗓️ Execution Sequence (Milestones)
1.  **[Completed] Core Foundation:** Implement `src/lib.rs`, defining the plugin trait and basic context management.
2.  **[Completed] Zsh Integration:** Write and test the necessary hooks to ensure state capture works within a real Zsh environment. Includes `generate_zshrc_snippet()` for .zshrc installation.
3.  **[Completed] Fish Integration:** Implement Fish shell hooks with `generate_config_snippet()` for config.fish installation.
4.  **[Completed] Starship Context Injection:** Develop logic to correctly export state variables for consumption by Starship via `SHELLAI_*` environment variables.
5.  **[Completed] Local AI Support:** Add ability to run local (non-API) AI models with optional API key and test-mode for testing.
6.  **[Completed] AI API Hook:** Connect the captured context to an external LLM API service and display suggestions interactively in the terminal. Includes `--ai-config` CLI flag.

### 📦 Module Structure

```
src/
├── lib.rs              # Main library exports
├── main.rs             # CLI binary with all command flags
├── plugin/
│   ├── mod.rs          # ShellPlugin trait, ShellContext, PluginConfig
│   └── mock_plugin.rs   # Mock implementation for testing
├── zsh/
│   └── mod.rs          # precmd, preexec, generate_zshrc_snippet
├── fish/
│   └── mod.rs          # prompt, command_not_found, generate_config_snippet
├── starship/
│   └── mod.rs          # export_context for SHELLAI_* env vars
└── ai/
    └── mod.rs          # AiIntegration, AiSuggestion, LLM integration
```

### 🔧 CLI Commands

| Command | Description |
|---------|-------------|
| `--hook precmd` | Run Zsh precmd hook |
| `--hook preexec <cmd>` | Run Zsh preexec hook |
| `--fish-hook prompt` | Run Fish prompt hook |
| `--fish-hook command_not_found <cmd>` | Run Fish command not found hook |
| `--install [path]` | Generate Zsh .zshrc snippet |
| `--fish-install [path]` | Generate Fish config.fish snippet |
| `--export-context` | Export context as SHELLAI_* vars |
| `--ai-config <endpoint> <api_key> [model]` | Configure and test AI suggestion |

---

*This plan is now finalized and serves as the blueprint for implementation.*

*Last updated: 2026-04-21*
