//! Core types for Shally framework

use std::collections::HashMap;

/// Represents the current shell context/state
#[derive(Debug, Clone)]
pub struct ShellContext {
    /// The last command that was executed
    pub last_command: Option<String>,
    /// Output from the last command (stdout/stderr)
    pub last_output: Option<String>,
    /// Exit code of the last command
    pub exit_code: Option<i32>,
    /// Current working directory
    pub cwd: String,
    /// Shell name (bash, zsh, fish)
    pub shell: String,
}

impl Default for ShellContext {
    fn default() -> Self {
        ShellContext {
            last_command: None,
            last_output: None,
            exit_code: None,
            cwd: "/".to_string(),
            shell: "bash".to_string(),
        }
    }
}

impl ShellContext {
    /// Create a new ShellContext with minimal info
    pub fn new(cwd: &str, shell: &str) -> Self {
        ShellContext {
            cwd: cwd.to_string(),
            shell: shell.to_string(),
            ..Default::default()
        }
    }

    /// Create context from environment variables
    pub fn from_env() -> Self {
        let cwd = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "/".to_string());
        
        let shell = std::env::var("SHELL")
            .unwrap_or_else(|_| "/bin/bash".to_string());
        
        let shell_name = shell.rsplit('/').next().unwrap_or("bash");
        
        ShellContext::new(&cwd, shell_name)
    }
}

/// Result of a command execution
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub output: String,
    pub exit_code: i32,
}
