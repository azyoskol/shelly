// src/cli/mod.rs
// CLI argument handling and main entry point orchestration

use crate::core::PluginConfig;
use crate::config;
use crate::commands;
use crate::hooks;
use crate::ai::AiIntegration;
use crate::history::HistoryManager;
use std::env;

/// Resolves configuration from explicit path, environment, or defaults
pub fn resolve_config(explicit_path: Option<&str>) -> PluginConfig {
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
pub fn handle_hook_commands(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 2 && args[1] == "--hook" {
        hooks::execute_zsh_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    if args.len() > 2 && args[1] == "--fish-hook" {
        hooks::execute_fish_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    None
}

/// Runs the default initialization mode
pub fn run_default_mode(base_config: &PluginConfig) {
    use crate::plugin::mock_plugin::MockPlugin;
    use crate::core::ShellPlugin;

    println!("Shally Framework: Initializing core...");

    let config = base_config.clone();
    println!("Configuration loaded for shell: {}", config.shell_name);

    let mut context = crate::core::ShellContext::new();

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
