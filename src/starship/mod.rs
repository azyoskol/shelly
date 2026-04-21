// src/starship/mod.rs
//! Starship integration layer.
//!
//! This module implements context injection for Starship prompt customization.
//! It exports standardized `SHELLAI_*` environment variables that Starship can read.

use crate::plugin::ShellContext;

const SHELLAI_PREFIX: &str = "SHELLAI_";

/// Exports the shell context as environment variables.
///
/// This function takes the internal shell context and exports it as
/// `SHELLAI_*` environment variables that Starship can consume.
pub fn export_context(context: &ShellContext) -> Vec<(String, String)> {
    context
        .iter()
        .map(|(key, value)| {
            let export_key = format!("{}{}", SHELLAI_PREFIX, key.to_uppercase());
            (export_key, value.clone())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_export_context() {
        let mut context = ShellContext::new();
        context.insert("CURRENT_DIR".to_string(), "/home/user".to_string());
        context.insert("GIT_BRANCH".to_string(), "main".to_string());

        let exports = export_context(&context);
        assert_eq!(exports.len(), 2);

        let export_map: HashMap<String, String> = exports.into_iter().collect();
        assert_eq!(export_map.get("SHELLAI_CURRENT_DIR"), Some(&"/home/user".to_string()));
        assert_eq!(export_map.get("SHELLAI_GIT_BRANCH"), Some(&"main".to_string()));
    }
}