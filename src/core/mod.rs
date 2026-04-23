//! Core module for Shally
//! Contains base types and traits used throughout the project

pub mod types;
pub mod plugin;

pub use types::{PluginConfig, ShellContext};
pub use plugin::ShellPlugin;
