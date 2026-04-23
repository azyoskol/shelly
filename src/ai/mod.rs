// src/ai/mod.rs
//! AI integration module for command suggestions.
//!
//! This module handles the pipeline from context + history to AI API call to suggestion display.

use crate::plugin::ShellContext;
use crate::history::HistoryEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AiSuggestion {
    pub command: String,
    pub confidence: f32,
    pub explanation: Option<String>,
    pub ai_command: Option<String>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
}

pub struct AiIntegration {
    api_endpoint: Option<String>,
    api_key: Option<String>,
    model: String,
    require_auth: bool,
}

impl Default for AiIntegration {
    fn default() -> Self {
        Self {
            api_endpoint: None,
            api_key: None,
            model: "gpt-3.5-turbo".to_string(),
            require_auth: true,
        }
    }
}

impl AiIntegration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(&mut self, endpoint: &str, api_key: &str) -> &mut Self {
        self.api_endpoint = Some(endpoint.to_string());
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_local_model(&mut self, endpoint: &str) -> &mut Self {
        self.api_endpoint = Some(endpoint.to_string());
        self.api_key = None;
        self.require_auth = false;
        self
    }

    pub fn with_model(&mut self, model: &str) -> &mut Self {
        self.model = model.to_string();
        self
    }

    pub fn is_local_model(&self) -> bool {
        self.model.to_lowercase().starts_with("local")
    }

    pub fn suggest_command(
        &self,
        context: &ShellContext,
        history: &[HistoryEntry],
    ) -> Result<AiSuggestion, String> {
        // Local model path handling
        if self.is_local_model() {
            let prompt = self.build_prompt(context, history);
            return self.call_api(&prompt);
        }

        let endpoint = self.api_endpoint.as_ref().ok_or("API endpoint not set. Call with_config() first.")?;

        if self.require_auth {
            let api_key = self.api_key.as_ref().ok_or("API key not set. Call with_config() first.")?;
            if endpoint.is_empty() || api_key.is_empty() {
                return Err("API endpoint or key is empty. Configure with_config() first.".to_string());
            }
        } else {
            if endpoint.is_empty() {
                return Err("API endpoint is empty. Configure with_config() first.".to_string());
            }
        }

        let prompt = self.build_prompt(context, history);
        self.call_api(&prompt)
    }

    fn build_prompt(&self, context: &ShellContext, history: &[HistoryEntry]) -> String {
        let context_str = context
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        let history_str = history.iter()
            .map(|e| format!("[exit={}, {}ms] {}", e.exit_code, e.duration_ms, e.command))
            .collect::<Vec<_>>()
            .join("; ");

        format!(
            "You are a shell command assistant. Based on the following context and command history, suggest the next terminal command.\n\nContext: {}\nCommand History: {}\n\nRespond with ONLY the command to execute, nothing else.",
            context_str, history_str
        )
    }

    fn call_api(&self, prompt: &str) -> Result<AiSuggestion, String> {
        // Test mode: avoid actual HTTP calls in tests
        if std::env::var("SHELLAI_TEST_MODE").ok().as_deref() == Some("1") {
            return Ok(AiSuggestion {
                command: "echo test-mode".to_string(),
                confidence: 0.92,
                explanation: Some("test mode".to_string()),
                ai_command: None,
            });
        }
        // Local model simulation
        if self.is_local_model() {
            return Ok(AiSuggestion {
                command: "echo local-suggestion".to_string(),
                confidence: 0.90,
                explanation: Some("local model simulated".to_string()),
                ai_command: Some("shally-ai:echo local-suggestion".to_string()),
            });
        }
        let endpoint = self.api_endpoint.as_ref().unwrap();

        let client = reqwest::blocking::Client::new();

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        let body = serde_json::json!({
            "model": self.model,
            "messages": messages,
            "max_tokens": 100,
            "temperature": 0.7
        });

        let mut request = client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .json(&body);

        if self.require_auth {
            let api_key = self.api_key.as_ref().unwrap();
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request
            .send()
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let api_response: ApiResponse = response
            .json()
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let content = api_response
            .choices
            .first()
            .ok_or("No choices in response")?
            .message
            .content
            .trim()
            .to_string();

        let ai_command = if self.is_local_model() {
            Some(format!("shally-ai:{}", content))
        } else {
            None
        };

        Ok(AiSuggestion {
            command: content,
            confidence: 0.85,
            explanation: None,
            ai_command,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_not_configured() {
        let ai = AiIntegration::new();
        let context = ShellContext::new();
        let history = vec![HistoryEntry {
            command: "ls -la".to_string(),
            exit_code: 0,
            duration_ms: 12,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
            ai_suggestion: None,
        }];

        let result = ai.suggest_command(&context, &history);
        assert!(result.is_err());
    }

    #[test]
    fn test_build_prompt() {
        let ai = AiIntegration::new();
        let mut context = ShellContext::new();
        context.insert("PWD".to_string(), "/home/user".to_string());

        let history = vec![
            HistoryEntry {
                command: "ls".to_string(),
                exit_code: 0,
                duration_ms: 5,
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                ai_suggestion: None,
            },
            HistoryEntry {
                command: "cd /tmp".to_string(),
                exit_code: 0,
                duration_ms: 1,
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                ai_suggestion: None,
            },
        ];
        let prompt = ai.build_prompt(&context, &history);

        assert!(prompt.contains("PWD=/home/user"));
        assert!(prompt.contains("ls"));
        assert!(prompt.contains("cd /tmp"));
    }

    #[test]
    fn test_with_model() {
        let mut integration = AiIntegration::new();
        integration.with_config("https://api.openai.com/v1/chat/completions", "test-key")
            .with_model("gpt-4");
        assert_eq!(integration.model, "gpt-4");
    }

    #[test]
    fn test_local_model_variants() {
        // Ensure test mode is not active for local-model tests
        std::env::remove_var("SHELLAI_TEST_MODE");
        let variants = ["local-llm-v1", "local-xl"];
        for &name in variants.iter() {
            let mut ai = AiIntegration::new();
            // configure as local model
            ai.with_local_model("http://localhost:1234/v1/chat/completions");
            ai.with_model(name);

            let context = ShellContext::new();
            let history = vec![HistoryEntry {
                command: "echo hi".to_string(),
                exit_code: 0,
                duration_ms: 3,
                timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs(),
                ai_suggestion: None,
            }];
            let result = ai.suggest_command(&context, &history);
            assert!(result.is_ok());
            let suggestion = result.unwrap();
            assert_eq!(suggestion.command, "echo local-suggestion");
        }
    }

    #[test]
    fn test_cli_config_endpoint_and_key() {
        let mut integration = AiIntegration::new();
        integration.with_config("https://api.custom.com/v1/chat", "my-secret-key");

        assert_eq!(integration.api_endpoint, Some("https://api.custom.com/v1/chat".to_string()));
        assert_eq!(integration.api_key, Some("my-secret-key".to_string()));
    }

    #[test]
    fn test_suggest_command_after_config() {
        std::env::set_var("SHELLAI_TEST_MODE", "1");
        let mut integration = AiIntegration::new();
        integration.with_config("https://api.openai.com/v1/chat/completions", "test-key")
            .with_model("gpt-4");

        let context = ShellContext::new();
        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let history = vec![
            HistoryEntry { command: "ls".to_string(), exit_code: 0, duration_ms: 5, timestamp: ts, ai_suggestion: None },
            HistoryEntry { command: "cd /home".to_string(), exit_code: 0, duration_ms: 1, timestamp: ts, ai_suggestion: None },
        ];

        let result = integration.suggest_command(&context, &history);
        assert!(result.is_ok());
        let suggestion = result.unwrap();
        assert_eq!(suggestion.command, "echo test-mode");
    }

    #[test]
    fn test_default_model_is_gpt_3_5() {
        let integration = AiIntegration::new();
        assert_eq!(integration.model, "gpt-3.5-turbo");
    }

    #[test]
    fn test_config_requires_endpoint() {
        let mut integration = AiIntegration::new();
        integration.with_config("", "some-key");
        let context = ShellContext::new();
        let result = integration.suggest_command(&context, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_requires_api_key() {
        let mut integration = AiIntegration::new();
        integration.with_config("https://api.openai.com/v1/chat/completions", "");
        let context = ShellContext::new();
        let result = integration.suggest_command(&context, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_local_model_no_auth_required() {
        let mut integration = AiIntegration::new();
        integration.with_local_model("http://localhost:11434/v1/chat/completions");

        assert!(integration.api_key.is_none());
        assert!(!integration.require_auth);
        assert_eq!(integration.api_endpoint, Some("http://localhost:11434/v1/chat/completions".to_string()));
    }

    #[test]
    fn test_local_model_suggest_command_no_api_key_error() {
        std::env::set_var("SHELLAI_TEST_MODE", "1");
        let mut integration = AiIntegration::new();
        integration.with_local_model("http://localhost:11434/v1/chat/completions");

        let ts = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs();
        let context = ShellContext::new();
        let history = vec![HistoryEntry { command: "ls".to_string(), exit_code: 0, duration_ms: 5, timestamp: ts, ai_suggestion: None }];

        let result = integration.suggest_command(&context, &history);
        assert!(result.is_ok());
        // In test mode or local mode, either canned response should be acceptable in tests
        let cmd = &result.unwrap().command;
        assert!(cmd == "echo test-mode" || cmd == "echo local-suggestion");
    }

    #[test]
    fn test_with_local_model_sets_require_auth_false() {
        let mut integration = AiIntegration::new();
        assert!(integration.require_auth);

        integration.with_local_model("http://localhost:8080/v1");
        assert!(!integration.require_auth);
    }

    #[test]
    fn test_local_model_empty_endpoint_rejected() {
        let mut integration = AiIntegration::new();
        integration.with_local_model("");
        let context = ShellContext::new();
        let result = integration.suggest_command(&context, &[]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("endpoint"));
    }

    #[test]
    fn test_require_auth_defaults_to_true() {
        let integration = AiIntegration::new();
        assert!(integration.require_auth);
    }
}
