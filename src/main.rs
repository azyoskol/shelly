use shally::plugin::{PluginConfig, ShellContext, ShellPlugin, mock_plugin::MockPlugin};
use shally::config;
use shally::zsh;
use shally::fish;
use shally::starship;
use shally::ai::AiIntegration;
use shally::history::HistoryManager;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Resolve config from --config flag, env var, or default locations
    let explicit_config = args.iter()
        .position(|a| a == "--config")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str());

    let base_config = resolve_config(explicit_config);

    if let Some(result) = handle_hook_commands(&args, &base_config) {
        return result;
    }

    if let Some(result) = handle_install_commands(&args) {
        return result;
    }

    if let Some(result) = handle_export_context_command(&args) {
        return result;
    }

    if let Some(result) = handle_ai_config_command(&args, &base_config) {
        return result;
    }

    if let Some(result) = handle_history_commands(&args, &base_config) {
        return result;
    }

    run_default_mode(&base_config);
}

/// Resolves configuration from explicit path, environment, or defaults
fn resolve_config(explicit_path: Option<&str>) -> PluginConfig {
    if let Some(path) = config::resolve_config_path(explicit_path) {
        match config::load_config_from_path(&std::path::PathBuf::from(path.clone())) {
            Some(c) => {
                println!("Configuration loaded from: {}", path.display());
                c
            }
            None => {
                eprintln!("Warning: Could not parse config file {}, using defaults.", path.display());
                PluginConfig::default()
            }
        }
    } else {
        PluginConfig::default()
    }
}

/// Handles --hook and --fish-hook commands
fn handle_hook_commands(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 2 && args[1] == "--hook" {
        execute_zsh_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    if args.len() > 2 && args[1] == "--fish-hook" {
        execute_fish_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    None
}

/// Handles --install and --fish-install commands
fn handle_install_commands(args: &[String]) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shally");
        println!("{}", zsh::generate_zshrc_snippet(binary_path));
        return Some(Ok(()));
    }

    if args.len() > 1 && args[1] == "--fish-install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shally");
        println!("{}", fish::generate_config_snippet(binary_path));
        return Some(Ok(()));
    }

    None
}

/// Handles --export-context command
fn handle_export_context_command(args: &[String]) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--export-context" {
        let mut context = ShellContext::new();
        context.insert(
            "CURRENT_DIR".to_string(),
            env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        let exports = starship::export_context(&context);
        for (key, value) in exports {
            println!("{}={}", key, value);
        }
        return Some(Ok(()));
    }

    None
}

/// Handles --ai-config command
fn handle_ai_config_command(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--ai-config" {
        execute_ai_config(args, base_config);
        return Some(Ok(()));
    }

    None
}

/// Handles --history command
fn handle_history_commands(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 2 && args[1] == "--history" {
        execute_history_command(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    None
}

/// Executes zsh hook based on hook type
fn execute_zsh_hook(hook_type: &str, command: Option<&str>, base_config: &PluginConfig) {
    let mut context = ShellContext::new();
    let config = PluginConfig {
        shell_name: "zsh".to_string(),
        settings: base_config.settings.clone(),
        ..base_config.clone()
    };
    let _ = MockPlugin::initialize(&config);

    match hook_type {
        "precmd" => {
            let _ = zsh::precmd::<MockPlugin>(&mut context);
        }
        "preexec" => {
            let _ = zsh::preexec::<MockPlugin>(command.unwrap_or(""), 0, &mut context);
        }
        _ => {
            eprintln!("Unknown hook type: {}", hook_type);
        }
    }
}

/// Executes fish hook based on hook type
fn execute_fish_hook(hook_type: &str, command: Option<&str>, base_config: &PluginConfig) {
    let mut context = ShellContext::new();
    let config = PluginConfig {
        shell_name: "fish".to_string(),
        settings: base_config.settings.clone(),
        ..base_config.clone()
    };
    let _ = MockPlugin::initialize(&config);

    match hook_type {
        "prompt" => {
            let _ = fish::prompt::<MockPlugin>(&mut context);
        }
        "command_not_found" => {
            let _ = fish::command_not_found::<MockPlugin>(command.unwrap_or(""), &mut context);
        }
        _ => {
            eprintln!("Unknown fish hook type: {}", hook_type);
        }
    }
}

/// Executes AI configuration and suggestion
fn execute_ai_config(args: &[String], base_config: &PluginConfig) {
    let endpoint = args.get(2).map(|s| s.as_str()).unwrap_or("https://api.openai.com/v1/chat/completions");
    let api_key = args.get(3).map(|s| s.as_str()).unwrap_or("");
    let model = args.get(4).map(|s| s.as_str()).unwrap_or("gpt-3.5-turbo");

    let mut ai = AiIntegration::new();
    ai.with_config(endpoint, api_key).with_model(model);

    let context = ShellContext::new();

    // Load history for AI context
    let history_path = base_config.settings.get("history_file").map(|s| s.as_str());
    let max_entries = base_config.settings.get("max_history")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);
    let history_mgr = HistoryManager::new(history_path).with_max_entries(max_entries);
    if let Err(e) = history_mgr.load() {
        eprintln!("Warning: Could not load history: {}", e);
    }

    let ai_history = history_mgr.get_relevant_for_ai(&context, 10);

    match ai.suggest_command(&context, &ai_history) {
        Ok(suggestion) => {
            println!("Command: {}", suggestion.command);
            if let Some(exp) = suggestion.explanation {
                println!("Explanation: {}", exp);
            }
            println!("Confidence: {:.2}", suggestion.confidence);
            if let Some(ref ai_cmd) = suggestion.ai_command {
                println!("AI Command: {}", ai_cmd);
            }
        }
        Err(e) => eprintln!("AI suggestion failed: {}", e),
    }
}

/// Executes history subcommands
fn execute_history_command(subcommand: &str, arg: Option<&str>, base_config: &PluginConfig) {
    let history_path = base_config.settings.get("history_file").map(|s| s.as_str());
    let max_entries = base_config.settings.get("max_history")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);
    let history_mgr = HistoryManager::new(history_path).with_max_entries(max_entries);

    match subcommand {
        "recent" => {
            let count = arg.and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
            if history_mgr.load().is_ok() {
                for entry in history_mgr.get_recent(count) {
                    println!(
                        "[{}] exit={} {}ms{}",
                        entry.command,
                        entry.exit_code,
                        entry.duration_ms,
                        entry.ai_suggestion.as_ref().map(|s| format!(" (AI: {})", s)).unwrap_or_default()
                    );
                }
            }
        }
        "search" => {
            let query = arg.unwrap_or("");
            if history_mgr.load().is_ok() {
                for entry in history_mgr.search(query) {
                    println!("[{}] exit={} {}ms", entry.command, entry.exit_code, entry.duration_ms);
                }
            }
        }
        "clear" => {
            if let Err(e) = history_mgr.clear() {
                eprintln!("Failed to clear history: {}", e);
            } else {
                println!("History cleared.");
            }
        }
        _ => {
            eprintln!("Usage: shally --history [recent|search|clear]");
        }
    }
}

/// Runs the default initialization mode
fn run_default_mode(base_config: &PluginConfig) {
    println!("Shally Framework: Initializing core...");

    let config = base_config.clone();
    println!("Configuration loaded for shell: {}", config.shell_name);

    let mut context = ShellContext::new();

    if let Ok(_) = MockPlugin::initialize(&config) {
        println!("✅ MockPlugin Initialized successfully.");
    } else {
        eprintln!("❌ Initialization failed.");
        return;
    }

    match MockPlugin::pre_prompt_hook(&mut context) {
        Ok(_) => println!("\n✨ Pre-Prompt Hook executed. Context updated: {:?}", context),
        Err(e) => eprintln!("Error during pre-hook: {}", e),
    }

    println!("\nShally Framework Initialized successfully.");
}
