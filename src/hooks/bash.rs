// src/hooks/bash.rs
// Bash hook execution handlers

use crate::bash;
use crate::core::{PluginConfig, ShellContext, ShellPlugin};
use crate::plugin::mock_plugin::MockPlugin;
use crate::errors::log_hook_execution;
use log::debug;

/// Executes bash hook based on hook type
pub fn execute_bash_hook(hook_type: &str, command: Option<&str>, base_config: &PluginConfig) {
    let mut context = ShellContext::new();
    let config = PluginConfig {
        shell_name: "bash".to_string(),
        settings: base_config.settings.clone(),
        ..base_config.clone()
    };
    
    log_hook_execution(hook_type, "bash");
    debug!("Executing bash hook: {} with command: {:?}", hook_type, command);
    
    let _ = MockPlugin::initialize(&config);

    match hook_type {
        "precmd" => {
            let _ = bash::precmd::<MockPlugin>(&mut context);
        }
        "preexec" => {
            let _ = bash::preexec::<MockPlugin>(command.unwrap_or(""), 0, &mut context);
        }
        _ => {
            eprintln!("Unknown bash hook type: {}", hook_type);
        }
    }
}

