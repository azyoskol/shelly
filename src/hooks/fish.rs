// src/hooks/fish.rs
// Fish hook execution handlers

use crate::fish;
use crate::core::{PluginConfig, ShellContext};
use crate::plugin::mock_plugin::MockPlugin;

/// Executes fish hook based on hook type
pub fn execute_fish_hook(hook_type: &str, command: Option<&str>, base_config: &PluginConfig) {
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
