// src/commands/ai_config.rs
// AI configuration command handler

use crate::ai::AiIntegration;
use crate::history::HistoryManager;
use crate::core::{PluginConfig, ShellContext};

/// Handles --ai-config command
pub fn handle_ai_config_command(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 1 && args[1] == "--ai-config" {
        execute_ai_config(args, base_config);
        return Some(Ok(()));
    }

    None
}

/// Executes AI configuration and suggestion
fn execute_ai_config(args: &[String], base_config: &PluginConfig) {
    let endpoint = args.get(2).map(|s| s.as_str()).unwrap_or("https://api.openai.com/v1/chat/completions");
    let api_key = args.get(3).map(|s| s.as_str()).unwrap_or("");
    let model = args.get(4).map(|s| s.as_str()).unwrap_or("gpt-3.5-turbo");

    let mut ai = AiIntegration::new();
    ai.with_config(endpoint, api_key).with_model(model);

    let context = ShellContext::new();

    // Load history for AI context
    let history_path = base_config.settings.get("history_file").map(|s| s.as_str());
    let max_entries = base_config.settings.get("max_history")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);
    let history_mgr = HistoryManager::new(history_path).with_max_entries(max_entries);
    if let Err(e) = history_mgr.load() {
        eprintln!("Warning: Could not load history: {}", e);
    }

    let ai_history = history_mgr.get_relevant_for_ai(&context, 10);

    match ai.suggest_command(&context, &ai_history) {
        Ok(suggestion) => {
            println!("Command: {}", suggestion.command);
            if let Some(exp) = suggestion.explanation {
                println!("Explanation: {}", exp);
            }
            println!("Confidence: {:.2}", suggestion.confidence);
            if let Some(ref ai_cmd) = suggestion.ai_command {
                println!("AI Command: {}", ai_cmd);
            }
        }
        Err(e) => eprintln!("AI suggestion failed: {}", e),
    }
}
