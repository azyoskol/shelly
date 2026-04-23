// src/plugin/mod.rs
pub mod mock_plugin;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// A simplified representation of shell environment variables
pub type ShellContext = HashMap<String, String>;

/// Configuration structure loaded from YAML file or defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    #[serde(default = "default_shell_name")]
    pub shell_name: String,
    #[serde(default)]
    pub settings: HashMap<String, String>,
}

fn default_shell_name() -> String {
    "zsh".to_string()
}

impl Default for PluginConfig {
    fn default() -> Self {
        PluginConfig {
            shell_name: default_shell_name(),
            settings: HashMap::new(),
        }
    }
}

/// The core trait that all plugins must implement to integrate with the shell.
pub trait ShellPlugin {
    /// Returns a unique name for the plugin (e.g., "git_status", "docker").
    fn name() -> &'static str;

    /// Initializes the plugin using loaded configuration settings.
    fn initialize(config: &PluginConfig) -> Result<(), String>;

    /// Executes logic before drawing the prompt, allowing context updates (e.g., Git branch).
    fn pre_prompt_hook(context: &mut ShellContext) -> Result<(), String>;

    /// Executes logic after a command finishes, used for result processing or cleanup.
    fn post_execute_hook(_command: &str, exit_code: i32) -> Result<(), String>;
}
