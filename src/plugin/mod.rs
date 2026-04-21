// src/plugin/mod.rs
pub mod mock_plugin;

use std::collections::HashMap;

/// A simplified representation of shell environment variables
pub type ShellContext = HashMap<String, String>;

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

/// Placeholder structure to hold loaded configuration data for plugins
pub struct PluginConfig {
    pub shell_name: String, // e.g., "zsh", "fish"
    pub settings: HashMap<String, String>,
}
