// src/commands/install.rs
// Installation command handlers

use crate::zsh;
use crate::fish;
use crate::bash;

/// Handles --install, --fish-install, and --bash-install commands
pub fn handle_install_commands(args: &[String]) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shally");
        println!("{}", zsh::generate_zshrc_snippet(binary_path));
        return Some(Ok(()));
    }

    if args.len() > 1 && args[1] == "--fish-install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shally");
        println!("{}", fish::generate_config_snippet(binary_path));
        return Some(Ok(()));
    }

    if args.len() > 1 && args[1] == "--bash-install" {
        let binary_path = args.get(2).map(|s| s.as_str()).unwrap_or("shally");
        println!("{}", bash::generate_bashrc_snippet(binary_path));
        return Some(Ok(()));
    }

    None
}
