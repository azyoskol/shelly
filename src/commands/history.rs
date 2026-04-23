// src/commands/history.rs
// History command handlers

use crate::history::HistoryManager;
use crate::core::PluginConfig;

/// Handles --history command
pub fn handle_history_commands(args: &[String], base_config: &PluginConfig) -> Option<Result<(), ()>> {
    if args.len() > 2 && args[1] == "--history" {
        execute_history_command(&args[2], args.get(3).map(|s| s.as_str()), base_config);
        return Some(Ok(()));
    }

    None
}

/// Executes history subcommands
pub fn execute_history_command(subcommand: &str, arg: Option<&str>, base_config: &PluginConfig) {
    let history_path = base_config.settings.get("history_file").map(|s| s.as_str());
    let max_entries = base_config.settings.get("max_history")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1000);
    let history_mgr = HistoryManager::new(history_path).with_max_entries(max_entries);

    match subcommand {
        "recent" => {
            let count = arg.and_then(|s| s.parse::<usize>().ok()).unwrap_or(10);
            if history_mgr.load().is_ok() {
                for entry in history_mgr.get_recent(count) {
                    println!(
                        "[{}] exit={} {}ms{}",
                        entry.command,
                        entry.exit_code,
                        entry.duration_ms,
                        entry.ai_suggestion.as_ref().map(|s| format!(" (AI: {})", s)).unwrap_or_default()
                    );
                }
            }
        }
        "search" => {
            let query = arg.unwrap_or("");
            if history_mgr.load().is_ok() {
                for entry in history_mgr.search(query) {
                    println!("[{}] exit={} {}ms", entry.command, entry.exit_code, entry.duration_ms);
                }
            }
        }
        "clear" => {
            if let Err(e) = history_mgr.clear() {
                eprintln!("Failed to clear history: {}", e);
            } else {
                println!("History cleared.");
            }
        }
        _ => {
            eprintln!("Usage: shally --history [recent|search|clear]");
        }
    }
}
