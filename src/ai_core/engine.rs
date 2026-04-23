use super::decision::AiDecision;
use crate::core::types::ShellContext;

/// Represents a parsed suggestion from AI
#[derive(Debug, Clone, PartialEq)]
pub struct AiSuggestion {
    /// Type of action: "execute", "explain", "warn"
    pub action: &'static str,
    /// The content (command or explanation text)
    pub content: String,
}

/// Main AI Engine for making decisions and parsing responses
pub struct AiEngine;

impl AiEngine {
    /// Determine if AI should intercept based on context
    pub fn should_intercept(ctx: &ShellContext) -> AiDecision {
        // Check for command not found (exit code 127)
        if let Some(code) = ctx.exit_code {
            if code == 127 {
                return AiDecision::intercept("command_not_found", 10);
            }
            
            // Check for general command failure
            if code != 0 {
                // Higher priority if there's error output
                let priority = if ctx.last_output.as_ref().map_or(false, |o| o.contains("error") || o.contains("Error")) {
                    8
                } else {
                    5
                };
                return AiDecision::intercept("command_failed", priority);
            }
        }
        
        // Check for dangerous commands (even if not executed yet - for preexec)
        if let Some(cmd) = &ctx.last_command {
            if Self::is_dangerous_command(cmd) {
                return AiDecision::intercept("dangerous_command", 9);
            }
        }
        
        // No interception needed
        AiDecision::no_intercept()
    }

    /// Parse AI response into structured suggestion
    pub fn parse_suggestion(response: &str) -> Option<AiSuggestion> {
        // Try to extract code block
        if let Some(cmd) = Self::extract_code_block(response) {
            return Some(AiSuggestion {
                action: "execute",
                content: cmd,
            });
        }
        
        // If no code block, treat as explanation
        if !response.trim().is_empty() {
            return Some(AiSuggestion {
                action: "explain",
                content: response.to_string(),
            });
        }
        
        None
    }

    /// Extract command from markdown code block
    fn extract_code_block(text: &str) -> Option<String> {
        // Look for ```bash, ```sh, ```zsh, ```fish, or just ```
        let markers = ["```bash", "```sh", "```zsh", "```fish", "```"];
        
        for marker in markers {
            if let Some(start) = text.find(marker) {
                let content_start = start + marker.len();
                if let Some(end) = text[content_start..].find("```") {
                    let cmd = text[content_start..content_start + end].trim();
                    if !cmd.is_empty() {
                        return Some(cmd.to_string());
                    }
                }
            }
        }
        
        // Also check for inline code with backticks
        if text.contains('`') {
            let parts: Vec<&str> = text.split('`').collect();
            if parts.len() >= 2 {
                let potential_cmd = parts[1].trim();
                // Only if it looks like a command (has spaces or common command chars)
                if potential_cmd.contains(' ') || potential_cmd.starts_with(char::is_alphabetic) {
                    return Some(potential_cmd.to_string());
                }
            }
        }
        
        None
    }

    /// Check if a command is potentially dangerous
    fn is_dangerous_command(cmd: &str) -> bool {
        let dangerous_patterns = [
            "rm -rf",
            "dd if=",
            "> /dev/",
            "chmod -R 777",
            "mkfs",
            ":(){ :|:& };:",  // fork bomb
        ];
        
        dangerous_patterns.iter().any(|&p| cmd.contains(p))
    }

    /// Mock method for AI query (to be replaced with actual API call)
    pub async fn query_ai(prompt: &str) -> Result<String, crate::errors::ShallyError> {
        // TODO: Implement actual API call to LLM
        // For now, return a mock response for testing
        log::info!("AI Query (mock): {}", prompt);
        Ok("This is a mock response. In production, this would call an LLM API.".to_string())
    }
}
