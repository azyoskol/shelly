use shally::plugin::{PluginConfig, ShellContext, ShellPlugin, mock_plugin::MockPlugin};
use shally::config;
use shally::zsh;
use shally::fish;
use shally::starship;
use shally::ai::AiIntegration;
use shally::history::HistoryManager;
use shally::cli;
use shally::commands;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Resolve config from --config flag, env var, or default locations
    let explicit_config = args.iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    let base_config = cli::resolve_config(explicit_config);

    if let Some(result) = cli::handle_hook_commands(&args, &base_config) {
        return result;
    }

    if let Some(result) = commands::handle_install_commands(&args) {
        return result;
    }

    if let Some(result) = commands::handle_export_context_command(&args) {
        return result;
    }

    if let Some(result) = commands::handle_ai_config_command(&args, &base_config) {
        return result;
    }

    if let Some(result) = commands::handle_history_commands(&args, &base_config) {
        return result;
    }

    cli::run_default_mode(&base_config);
}
