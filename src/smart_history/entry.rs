use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single command history entry with metadata
#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// The command that was executed
    pub command: String,
    /// Working directory where command was executed
    pub cwd: String,
    /// Exit code of the command
    pub exit_code: i32,
    /// Timestamp when command was executed
    pub timestamp: u64,
    /// How many times this exact command was run
    pub frequency: u32,
}

impl HistoryEntry {
    /// Create a new history entry
    pub fn new(command: &str, cwd: &str, exit_code: i32) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        HistoryEntry {
            command: command.to_string(),
            cwd: cwd.to_string(),
            exit_code,
            timestamp,
            frequency: 1,
        }
    }

    /// Check if this entry matches a search query (fuzzy)
    pub fn matches(&self, query: &str) -> bool {
        self.command.to_lowercase().contains(&query.to_lowercase())
    }

    /// Increment frequency counter
    pub fn increment_frequency(&mut self) {
        self.frequency += 1;
    }
}
