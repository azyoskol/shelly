// src/lib.rs
pub mod core;
pub mod plugin;
pub mod config;
pub mod commands;
pub mod hooks;
pub mod cli;
pub mod errors;
pub mod ai_core;
pub mod smart_history;

// Legacy re-exports for compatibility
pub use core::types;
pub use plugin::ShellPlugin;