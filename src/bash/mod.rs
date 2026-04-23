// src/bash/mod.rs
//! Bash shell integration layer.
//!
//! This module implements hooks compatible with Bash's native hook system,
//! enabling context capture before prompts and after command execution.

use crate::plugin::{ShellPlugin, ShellContext};

/// Simulates Bash's PROMPT_COMMAND behavior (pre-prompt).
///
/// # Arguments
/// * `context` - Mutable reference to the shared shell context.
///
/// # Returns
/// Result indicating success or failure of hook execution.
pub fn precmd<P: ShellPlugin>(context: &mut ShellContext) -> Result<(), String> {
    P::pre_prompt_hook(context)
}

/// Simulates Bash's DEBUG trap behavior (pre-execution).
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

/// Generates Bash hook installation code for .bashrc.
///
/// This function outputs the bashrc snippet that users can add to their
/// ~/.bashrc to enable Shally hooks in their actual Bash environment.
pub fn generate_bashrc_snippet(binary_path: &str) -> String {
    format!(
        r#"# Shally Framework - Bash Integration
# Add this to your ~/.bashrc to enable Shally hooks

# Pre-prompt hook (runs before each prompt)
shally_precmd() {{
    {binary_path} --bash-hook precmd
}}

# Pre-execution hook (runs before each command)
shally_preexec() {{
    {binary_path} --bash-hook preexec "$BASH_COMMAND"
}}

# Store the previous PROMPT_COMMAND if it exists
if [ -n "$PROMPT_COMMAND" ]; then
    shally_old_precmd="$PROMPT_COMMAND"
fi

# Set up PROMPT_COMMAND to run our hook
PROMPT_COMMAND="shally_precmd"

# Set up DEBUG trap for pre-execution
trap 'shally_preexec' DEBUG
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
    fn test_bash_precmd_hook() {
        let mut context = HashMap::new();
        let result = precmd::<MockPlugin>(&mut context);
        assert!(result.is_ok());
        assert_eq!(
            context.get("CURRENT_DIR"),
            Some(&"test/path".to_string())
        );
    }

    #[test]
    fn test_bash_preexec_hook() {
        let mut context = HashMap::new();
        let command = "echo hello";
        let exit_code = 0;

        let result = preexec::<MockPlugin>(command, exit_code, &mut context);
        assert!(result.is_ok());
        assert_eq!(context.get("LAST_COMMAND"), Some(&command.to_string()));
        assert_eq!(context.get("EXIT_CODE"), Some(&exit_code.to_string()));
    }

    #[test]
    fn test_generate_bashrc_snippet() {
        let snippet = generate_bashrc_snippet("/usr/local/bin/shally");
        assert!(snippet.contains("shally_precmd"));
        assert!(snippet.contains("shally_preexec"));
        assert!(snippet.contains("/usr/local/bin/shally"));
        assert!(snippet.contains("PROMPT_COMMAND"));
        assert!(snippet.contains("DEBUG"));
        assert!(snippet.contains("trap"));
    }
}
