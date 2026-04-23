// src/lib.rs
pub mod core;
pub mod plugin;
pub mod config;
pub mod zsh;
pub mod fish;
pub mod starship;
pub mod ai;
pub mod history;
pub mod commands;
pub mod hooks;
pub mod cli;

// Re-export commonly used types at the crate root for convenience
pub use core::{PluginConfig, ShellContext, ShellPlugin};
