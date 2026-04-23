// src/hooks/mod.rs
// Hook execution handlers module

pub mod zsh;
pub mod fish;

pub use zsh::execute_zsh_hook;
pub use fish::execute_fish_hook;
