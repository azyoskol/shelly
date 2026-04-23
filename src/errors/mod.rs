// src/errors/mod.rs
// Error handling and logging utilities for Shally Framework

use std::fmt;
use log::{error, warn, info, debug};

/// Centralized error type for Shally Framework
#[derive(Debug)]
pub enum ShallyError {
    ConfigNotFound(String),
    ConfigParseError(String),
    IoError(std::io::Error),
    NetworkError(String),
    PluginError(String),
    HookError(String),
    HistoryError(String),
    AiError(String),
}

impl fmt::Display for ShallyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShallyError::ConfigNotFound(path) => write!(f, "Configuration file not found: {}", path),
            ShallyError::ConfigParseError(msg) => write!(f, "Failed to parse configuration: {}", msg),
            ShallyError::IoError(err) => write!(f, "I/O error: {}", err),
            ShallyError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ShallyError::PluginError(msg) => write!(f, "Plugin error: {}", msg),
            ShallyError::HookError(msg) => write!(f, "Hook execution error: {}", msg),
            ShallyError::HistoryError(msg) => write!(f, "History error: {}", msg),
            ShallyError::AiError(msg) => write!(f, "AI integration error: {}", msg),
        }
    }
}

impl std::error::Error for ShallyError {}

impl From<std::io::Error> for ShallyError {
    fn from(err: std::io::Error) -> Self {
        ShallyError::IoError(err)
    }
}

/// Result type alias for Shally operations
pub type ShallyResult<T> = Result<T, ShallyError>;

/// Initialize logging for the application
pub fn init_logging() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
}

/// Log configuration loading
pub fn log_config_loaded(path: &str) {
    info!("Configuration loaded from: {}", path);
}

/// Log configuration warning
pub fn log_config_warning(path: &str, reason: &str) {
    warn!("Configuration warning for {}: {}", path, reason);
}

/// Log hook execution
pub fn log_hook_execution(hook_type: &str, shell: &str) {
    debug!("Executing {} hook for {}", hook_type, shell);
}

/// Log command execution
pub fn log_command(command: &str) {
    debug!("Executing command: {}", command);
}

/// Log AI suggestion
pub fn log_ai_suggestion(model: &str, confidence: f32) {
    info!("AI suggestion generated using {} with confidence {:.2}", model, confidence);
}

/// Log history operation
pub fn log_history_operation(operation: &str, count: usize) {
    debug!("History {}: {} entries", operation, count);
}
