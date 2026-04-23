use crate::core::types::ShellContext;

/// Builder for creating contextual prompts to send to AI
pub struct PromptBuilder;

impl PromptBuilder {
    /// Build a help prompt based on the shell context
    pub fn build_help_prompt(ctx: &ShellContext) -> String {
        let mut prompt = String::from("You are a helpful shell assistant. ");
        
        // Add command info
        if let Some(cmd) = &ctx.last_command {
            prompt.push_str(&format!("The user ran: `{}`. ", cmd));
        } else {
            prompt.push_str("The user's last command is unknown. ");
        }
        
        // Add output/error info
        if let Some(output) = &ctx.last_output {
            prompt.push_str(&format!("Output/Error: {}. ", output));
        }
        
        // Add exit code
        if let Some(code) = ctx.exit_code {
            if code != 0 {
                prompt.push_str(&format!("Exit code: {} (error). ", code));
            }
        }
        
        // Add directory context
        prompt.push_str(&format!("Current directory: {}. ", ctx.cwd));
        
        // Add shell info
        prompt.push_str(&format!("Shell: {}. ", ctx.shell));
        
        // Add instruction
        prompt.push_str("Please provide a concise fix, explanation, or suggestion. ");
        prompt.push_str("If suggesting a command, format it in markdown code blocks with the shell name.");
        
        prompt
    }

    /// Build a prompt for explaining a command before execution (preexec hook)
    pub fn build_explain_prompt(command: &str, cwd: &str) -> String {
        format!(
            "Explain what this command does in simple terms: `{}`. \
             Current directory: {}. \
             Warn if the command is potentially dangerous.",
            command, cwd
        )
    }

    /// Build a prompt for suggesting alternatives (command not found)
    pub fn build_not_found_prompt(command: &str, shell: &str) -> String {
        format!(
            "The command `{}` was not found in {}. \
             Suggest: 1) How to install it, 2) Alternative commands, 3) Common typos.",
            command, shell
        )
    }
}
