// src/fish/mod.rs
//! Fish shell integration layer.
//!
//! This module implements hooks compatible with Fish's native event system,
//! enabling context capture before prompts and after command execution.

use crate::plugin::{ShellPlugin, ShellContext};

/// Simulates Fish's fish_prompt event.
pub fn prompt<P: ShellPlugin>(context: &mut ShellContext) -> Result<(), String> {
    P::pre_prompt_hook(context)
}

/// Simulates Fish's fish_command_not_found event.
pub fn command_not_found<P: ShellPlugin>(
    command: &str,
    context: &mut ShellContext,
) -> Result<(), String> {
    context.insert("FAILED_COMMAND".to_string(), command.to_string());
    P::post_execute_hook(command, 127)
}

/// Generates Fish hook installation code for config.fish.
///
/// This function outputs the config.fish snippet that users can add to their
/// ~/.config/fish/config.fish to enable ShellAI hooks in their Fish environment.
pub fn generate_config_snippet(binary_path: &str) -> String {
    format!(
        r#"# ShellAI Framework - Fish Integration
# Add this to your ~/.config/fish/config.fish to enable ShellAI hooks

# Pre-prompt hook
function shellai_prompt
    {binary_path} --fish-hook prompt
end

# Command not found handler
function shellai_command_not_found
    {binary_path} --fish-hook command_not_found $argv[1]
end

# Register the prompt hook
fish_prompt += shellai_prompt
"#,
        binary_path = binary_path
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use crate::plugin::mock_plugin::MockPlugin;

    #[test]
    fn test_fish_prompt_hook() {
        let mut context = HashMap::new();
        let result = prompt::<MockPlugin>(&mut context);
        assert!(result.is_ok());
        assert_eq!(
            context.get("CURRENT_DIR"),
            Some(&"test/path".to_string())
        );
    }

    #[test]
    fn test_fish_command_not_found() {
        let mut context = HashMap::new();
        let result = command_not_found::<MockPlugin>("nonexistent_cmd", &mut context);
        assert!(result.is_err());
        assert_eq!(context.get("FAILED_COMMAND"), Some(&"nonexistent_cmd".to_string()));
    }

    #[test]
    fn test_generate_config_snippet() {
        let snippet = generate_config_snippet("/usr/local/bin/shellai");
        assert!(snippet.contains("shellai_prompt"));
        assert!(snippet.contains("shellai_command_not_found"));
        assert!(snippet.contains("/usr/local/bin/shellai"));
        assert!(snippet.contains("fish_prompt"));
    }
}