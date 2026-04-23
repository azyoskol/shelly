//! Core plugin trait for Shally

use crate::core::types::ShellContext;

/// Main trait that all Shally plugins must implement
pub trait ShellPlugin {
    /// Get the plugin name
    fn name(&self) -> &str;
    
    /// Get the plugin version
    fn version(&self) -> &str {
        "0.1.0"
    }
    
    /// Called before a command is executed (preexec hook)
    /// Return Some(String) to modify the command, None to leave unchanged
    fn preexec(&self, _ctx: &ShellContext, command: &str) -> Option<String> {
        log::debug!("Plugin {} preexec: {}", self.name(), command);
        None
    }
    
    /// Called after a command is executed (precmd hook)
    /// Can be used to update state or provide feedback
    fn precmd(&self, _ctx: &ShellContext) {
        log::debug!("Plugin {} precmd", self.name());
    }
    
    /// Handle command not found errors
    /// Return Some(suggestion) if plugin can help
    fn handle_not_found(&self, _ctx: &ShellContext, _command: &str) -> Option<String> {
        None
    }
    
    /// Handle command execution errors
    /// Return Some(suggestion) if plugin can help
    fn handle_error(&self, _ctx: &ShellContext, _output: &str) -> Option<String> {
        None
    }
}
