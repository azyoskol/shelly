// src/zsh/mod.rs
//! Zsh-specific integration layer.
//!
//! This module implements hooks compatible with Zsh's native hook system,
//! enabling context capture before prompts and after command execution.

use crate::plugin::{ShellPlugin, ShellContext};

/// Simulates Zsh's precmd hook behavior.
///
/// # Arguments
/// * `context` - Mutable reference to the shared shell context.
///
/// # Returns
/// Result indicating success or failure of hook execution.
pub fn precmd<P: ShellPlugin>(context: &mut ShellContext) -> Result<(), String> {
    P::pre_prompt_hook(context)
}

/// Simulates Zsh's preexec hook behavior.
///
/// # Arguments
/// * `command` - The command being executed.
/// * `exit_code` - Exit code from the last command.
/// * `context` - Mutable reference to the shared shell context.
pub fn preexec<P: ShellPlugin>(
    command: &str,
    exit_code: i32,
    context: &mut ShellContext,
) -> Result<(), String> {
    context.insert("LAST_COMMAND".to_string(), command.to_string());
    context.insert("EXIT_CODE".to_string(), exit_code.to_string());

    P::post_execute_hook(command, exit_code)
}

/// Generates Zsh hook installation code for .zshrc.
///
/// This function outputs the zshrc snippet that users can add to their
/// ~/.zshrc to enable ShellAI hooks in their actual Zsh environment.
pub fn generate_zshrc_snippet(binary_path: &str) -> String {
    format!(
        r#"# ShellAI Framework - Zsh Integration
# Add this to your ~/.zshrc to enable ShellAI hooks

# Load shellai binary on precmd
precmd_shellai() {{
    {binary_path} --hook precmd
}}

# Load shellai binary on preexec
preexec_shellai() {{
    {binary_path} --hook preexec "$1"
}}

autoload -Uz precmd_functions
autoload -Uz preexec_functions

precmd_functions+=(precmd_shellai)
preexec_functions+=(preexec_shellai)
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
    fn test_zsh_precmd_hook() {
        let mut context = HashMap::new();
        let result = precmd::<MockPlugin>(&mut context);
        assert!(result.is_ok());
        assert_eq!(
            context.get("CURRENT_DIR"),
            Some(&"test/path".to_string())
        );
    }

    #[test]
    fn test_zsh_preexec_hook() {
        let mut context = HashMap::new();
        let command = "echo hello";
        let exit_code = 0;

        let result = preexec::<MockPlugin>(command, exit_code, &mut context);
        assert!(result.is_ok());
        assert_eq!(context.get("LAST_COMMAND"), Some(&command.to_string()));
        assert_eq!(context.get("EXIT_CODE"), Some(&exit_code.to_string()));
    }

    #[test]
    fn test_generate_zshrc_snippet() {
        let snippet = generate_zshrc_snippet("/usr/local/bin/shellai");
        assert!(snippet.contains("precmd_shellai"));
        assert!(snippet.contains("preexec_shellai"));
        assert!(snippet.contains("/usr/local/bin/shellai"));
        assert!(snippet.contains("precmd_functions"));
        assert!(snippet.contains("preexec_functions"));
    }
}