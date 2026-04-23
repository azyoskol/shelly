// src/lib.rs
// Shally Framework library root

pub mod core;
pub mod plugin;
pub mod config;
pub mod zsh;
pub mod fish;
pub mod bash;
pub mod starship;
pub mod ai;
pub mod history;
pub mod commands;
pub mod hooks;
pub mod cli;
pub mod errors;

// Re-export commonly used types at the crate root for convenience
pub use core::{PluginConfig, ShellContext, ShellPlugin};
pub use errors::{ShallyError, ShallyResult};
https://github.com/azyoskol/shelly/pull/6/conflict?name=src%252Ferrors%252Fmod.rs&base_oid=f7720f5aadb3a1631a8794e2679664ef0d029a3c&head_oid=497a974372e5f5862d4085b713037724970f681f