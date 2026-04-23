// src/cli/mod.rs
// CLI argument handling and main entry point orchestration

use crate::core::PluginConfig;
use crate::config;
use crate::commands;
use crate::hooks;
use crate::ai::AiIntegration;
use crate::history::HistoryManager;
use crate::errors::{ShallyResult, log_config_loaded, log_config_warning};
use std::env;
use log::{info, warn};

/// Resolves configuration from explicit path, environment, or defaults
pub fn resolve_config(explicit_path: Option<&str>) -> PluginConfig {
    if let Some(path) = config::resolve_config_path(explicit_path) {
        match config::load_config_from_path(&std::path::PathBuf::from(path.clone())) {
            Some(c) => {
                log_config_loaded(&path.display().to_string());
                c
            }
            None => {
                log_config_warning(&path.display().to_string(), "Could not parse config file");
                PluginConfig::default()
            }
        }
    } else {
        PluginConfig::default()
    }
}

/// Handles --hook, --fish-hook, and --bash-hook commands
pub fn handle_hook_commands(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 2 && args[1] == "--hook" {
        hooks::execute_zsh_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    if args.len() > 2 && args[1] == "--fish-hook" {
        hooks::execute_fish_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    if args.len() > 2 && args[1] == "--bash-hook" {
        hooks::execute_bash_hook(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    None
}

/// Runs the default initialization mode
pub fn run_default_mode(base_config: &PluginConfig) {
    use crate::plugin::mock_plugin::MockPlugin;
    use crate::core::ShellPlugin;

    info!("Shally Framework: Initializing core...");
    println!("Shally Framework: Initializing core...");

    let config = base_config.clone();
    info!("Configuration loaded for shell: {}", config.shell_name);
    println!("Configuration loaded for shell: {}", config.shell_name);

    let mut context = crate::core::ShellContext::new();

    if let Ok(_) = MockPlugin::initialize(&config) {
        info!("MockPlugin initialized successfully");
        println!("✅ MockPlugin Initialized successfully.");
    } else {
        eprintln!("❌ Initialization failed.");
        return;
    }

    match MockPlugin::pre_prompt_hook(&mut context) {
        Ok(_) => {
            info!("Pre-prompt hook executed successfully");
            println!("\n✨ Pre-Prompt Hook executed. Context updated: {:?}", context);
        }
        Err(e) => {
            warn!("Error during pre-hook: {}", e);
            eprintln!("Error during pre-hook: {}", e);
        }
    }

    info!("Shally Framework initialized successfully");
    println!("\nShally Framework Initialized successfully.");
}
