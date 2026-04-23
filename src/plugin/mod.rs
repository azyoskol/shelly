// src/plugin/mod.rs
// Plugin module - re-exports core types for backward compatibility

pub mod mock_plugin;

// Re-export from core for backward compatibility
pub use crate::core::{PluginConfig, ShellContext, ShellPlugin};
