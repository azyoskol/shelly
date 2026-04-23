// src/commands/history.rs
// History command handlers

use crate::history::HistoryManager;
use crate::core::PluginConfig;
use crate::errors::log_history_operation;
use log::{info, warn, debug};

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
            debug!("Getting recent {} history entries", count);
            
            if history_mgr.load().is_ok() {
                let entries = history_mgr.get_recent(count);
                log_history_operation("recent", entries.len());
                info!("Retrieved {} recent history entries", entries.len());
                
                for entry in entries {
                    println!(
                        "[{}] exit={} {}ms{}",
                        entry.command,
                        entry.exit_code,
                        entry.duration_ms,
                        entry.ai_suggestion.as_ref().map(|s| format!(" (AI: {})", s)).unwrap_or_default()
                    );
                }
            } else {
                warn!("Failed to load history");
            }
        }
        "search" => {
            let query = arg.unwrap_or("");
            debug!("Searching history for: {}", query);
            
            if history_mgr.load().is_ok() {
                let results = history_mgr.search(query);
                log_history_operation("search", results.len());
                info!("Found {} matching history entries", results.len());
                
                for entry in results {
                    println!("[{}] exit={} {}ms", entry.command, entry.exit_code, entry.duration_ms);
                }
            } else {
                warn!("Failed to load history");
            }
        }
        "clear" => {
            debug!("Clearing history");
            
            match history_mgr.clear() {
                Ok(_) => {
                    log_history_operation("clear", 0);
                    info!("History cleared successfully");
                    println!("History cleared.");
                }
                Err(e) => {
                    warn!("Failed to clear history: {}", e);
                    eprintln!("Failed to clear history: {}", e);
                }
            }
        }
        _ => {
            warn!("Unknown history subcommand: {}", subcommand);
            eprintln!("Usage: shally --history [recent|search|clear]");
        }
    }
}
