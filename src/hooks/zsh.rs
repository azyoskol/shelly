// src/hooks/zsh.rs
// Zsh hook execution handlers

use crate::zsh;
use crate::core::{PluginConfig, ShellContext, ShellPlugin};
use crate::plugin::mock_plugin::MockPlugin;
use crate::errors::log_hook_execution;
use log::debug;

/// Executes zsh hook based on hook type
pub fn execute_zsh_hook(hook_type: &str, command: Option<&str>, base_config: &PluginConfig) {
    let mut context = ShellContext::new();
    let config = PluginConfig {
        shell_name: "zsh".to_string(),
        settings: base_config.settings.clone(),
        ..base_config.clone()
    };
    
    log_hook_execution(hook_type, "zsh");
    debug!("Executing zsh hook: {} with command: {:?}", hook_type, command);
    
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
