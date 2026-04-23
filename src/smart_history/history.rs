use super::entry::HistoryEntry;
use super::rank::SearchRank;
use std::collections::HashMap;

/// Smart history manager with fuzzy search and context awareness
pub struct SmartHistory {
    entries: Vec<HistoryEntry>,
    command_index: HashMap<String, usize>, // Maps command to index in entries
}

impl SmartHistory {
    /// Create a new empty history
    pub fn new() -> Self {
        SmartHistory {
            entries: Vec::new(),
            command_index: HashMap::new(),
        }
    }

    /// Add a command to history
    pub fn add(&mut self, entry: HistoryEntry) {
        let cmd = entry.command.clone();
        
        // Check if this exact command exists
        if let Some(&idx) = self.command_index.get(&cmd) {
            // Increment frequency of existing entry
            self.entries[idx].increment_frequency();
        } else {
            // Add new entry
            let idx = self.entries.len();
            self.command_index.insert(cmd, idx);
            self.entries.push(entry);
        }
    }

    /// Search history with fuzzy matching and ranking
    pub fn search(&self, query: &str, cwd_hint: Option<&str>) -> Vec<SearchRank> {
        let mut results: Vec<SearchRank> = self.entries
            .iter()
            .filter(|e| e.matches(query))
            .map(|e| {
                let mut score = 0.0;
                
                // Base score for matching
                score += 10.0;
                
                // Bonus for exact match
                if e.command.to_lowercase() == query.to_lowercase() {
                    score += 20.0;
                }
                
                // Bonus for frequency (more used = more relevant)
                score += e.frequency as f64 * 2.0;
                
                // Bonus for directory context match
                if let Some(hint) = cwd_hint {
                    if e.cwd.starts_with(hint) || hint.starts_with(&e.cwd) {
                        score += 15.0;
                    }
                }
                
                // Recency bonus (newer commands slightly preferred)
                // Simplified: just use frequency as proxy for recency
                
                SearchRank::new(e.command.clone(), e.cwd.clone(), score, e.frequency)
            })
            .collect();
        
        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        results
    }

    /// Get most frequent commands
    pub fn get_frequent_commands(&self, limit: usize) -> Vec<(String, u32)> {
        let mut freqs: Vec<(String, u32)> = self.entries
            .iter()
            .map(|e| (e.command.clone(), e.frequency))
            .collect();
        
        freqs.sort_by(|a, b| b.1.cmp(&a.1));
        freqs.truncate(limit);
        freqs
    }

    /// Get recent commands
    pub fn get_recent(&self, limit: usize) -> Vec<&HistoryEntry> {
        let mut sorted: Vec<&HistoryEntry> = self.entries.iter().collect();
        sorted.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        sorted.truncate(limit);
        sorted
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.entries.clear();
        self.command_index.clear();
    }

    /// Get total count of unique commands
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for SmartHistory {
    fn default() -> Self {
        Self::new()
    }
}
