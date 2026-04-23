// src/commands/context.rs
// Context export command handler

use std::env;
use crate::starship;
use crate::core::ShellContext;

/// Handles --export-context command
pub fn handle_export_context_command(args: &[String]) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--export-context" {
        let mut context = ShellContext::new();
        context.insert(
            "CURRENT_DIR".to_string(),
            env::current_dir()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
        );
        let exports = starship::export_context(&context);
        for (key, value) in exports {
            println!("{}={}", key, value);
        }
        return Some(Ok(()));
    }

    None
}
