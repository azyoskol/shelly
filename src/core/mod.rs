// src/core/mod.rs
// Core module for Shally framework

pub mod types;
pub mod plugin;

pub use types::{PluginConfig, ShellContext};
pub use plugin::ShellPlugin;
