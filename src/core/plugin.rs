// src/core/plugin.rs
// Core plugin trait and functionality

use crate::core::types::{PluginConfig, ShellContext};

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
