// src/plugin/mock_plugin.rs
use crate::plugin::{ShellContext, ShellPlugin, PluginConfig};

#[derive(Default)]
pub struct MockPlugin;

impl ShellPlugin for MockPlugin {
    fn name() -> &'static str {
        "mock_plugin"
    }

    fn initialize(config: &PluginConfig) -> Result<(), String> {
        if config.shell_name.is_empty() {
            return Err("Configuration must specify a shell.".to_string());
        }
        Ok(())
    }

    fn pre_prompt_hook(context: &mut ShellContext) -> Result<(), String> {
        // Simulate updating context (e.g., adding the current directory)
        context.insert("CURRENT_DIR".to_string(), "test/path".to_string());
        Ok(())
    }

    fn post_execute_hook(_command: &str, exit_code: i32) -> Result<(), String> {
        // Simulate success check
        if exit_code != 0 {
            return Err(format!("Command failed with code {}", exit_code));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_plugin_lifecycle() -> Result<(), String> {
        let config = PluginConfig {
            shell_name: "zsh".to_string(),
            settings: HashMap::new(),
        };

        // 1. Initialization Check
        MockPlugin::initialize(&config)?;

        // 2. Pre-Prompt Hook Simulation
        let mut context: ShellContext = HashMap::new();
        MockPlugin::pre_prompt_hook(&mut context)?;

        // Verify if the context variable was correctly updated by the hook
        assert!(context.contains_key("CURRENT_DIR"));
        assert_eq!(context.get("CURRENT_DIR").unwrap(), "test/path");

        // 3. Post-Execution Hook Simulation (Success)
        MockPlugin::post_execute_hook("ls -l", 0)?;

        Ok(())
    }
}
