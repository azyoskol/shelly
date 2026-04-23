use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

const DEFAULT_HISTORY_FILE: &str = ".shellai_history.json";
const DEFAULT_MAX_ENTRIES: usize = 1000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: u64,
    pub exit_code: i32,
    pub ai_suggestion: Option<String>,
    pub duration_ms: u64,
}

impl HistoryEntry {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            exit_code: -1,
            ai_suggestion: None,
            duration_ms: 0,
        }
    }

    pub fn with_ai_suggestion(mut self, suggestion: &str) -> Self {
        self.ai_suggestion = Some(suggestion.to_string());
        self
    }

    pub fn mark_completed(&mut self, exit_code: i32, duration_ms: u64) {
        self.exit_code = exit_code;
        self.duration_ms = duration_ms;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HistoryStorage {
    entries: Vec<HistoryEntry>,
}

impl Default for HistoryStorage {
    fn default() -> Self {
        Self { entries: Vec::new() }
    }
}

pub struct HistoryManager {
    storage: RwLock<HistoryStorage>,
    history_path: PathBuf,
    max_entries: usize,
}

impl HistoryManager {
    pub fn new(history_path: Option<&str>) -> Self {
        let path = match history_path {
            Some(p) => PathBuf::from(p),
            None => Self::default_history_path(),
        };
        Self {
            storage: RwLock::new(HistoryStorage::default()),
            history_path: path,
            max_entries: DEFAULT_MAX_ENTRIES,
        }
    }

    pub fn with_max_entries(mut self, max: usize) -> Self {
        self.max_entries = max;
        self
    }

    fn default_history_path() -> PathBuf {
        std::env::var("HOME")
            .map(|h| PathBuf::from(h).join(DEFAULT_HISTORY_FILE))
            .unwrap_or_else(|_| PathBuf::from(DEFAULT_HISTORY_FILE))
    }

    pub fn load(&self) -> Result<usize, String> {
        if !self.history_path.exists() {
            return Ok(0);
        }
        let contents = fs::read_to_string(&self.history_path)
            .map_err(|e| format!("Failed to read history file: {}", e))?;
        let storage: HistoryStorage = serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse history file: {}", e))?;
        let mut guard = self.storage.write().map_err(|e| format!("Lock error: {}", e))?;
        guard.entries = storage.entries;
        Ok(guard.entries.len())
    }

    pub fn save(&self) -> Result<(), String> {
        let guard = self.storage.read().map_err(|e| format!("Lock error: {}", e))?;
        let json = serde_json::to_string_pretty(&*guard)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;
        fs::write(&self.history_path, json)
            .map_err(|e| format!("Failed to write history file: {}", e))?;
        Ok(())
    }

    pub fn add_entry(&self, entry: HistoryEntry) -> Result<(), String> {
        let mut guard = self.storage.write().map_err(|e| format!("Lock error: {}", e))?;
        guard.entries.push(entry);
        if guard.entries.len() > self.max_entries {
            let remove_count = guard.entries.len() - self.max_entries;
            guard.entries.drain(0..remove_count);
        }
        Ok(())
    }

    pub fn get_recent(&self, count: usize) -> Vec<HistoryEntry> {
        let guard = match self.storage.read() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        let start = if guard.entries.len() > count {
            guard.entries.len() - count
        } else {
            0
        };
        guard.entries[start..].to_vec()
    }

    pub fn search(&self, query: &str) -> Vec<HistoryEntry> {
        let guard = match self.storage.read() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        let lower = query.to_lowercase();
        guard.entries.iter()
            .filter(|e| e.command.to_lowercase().contains(&lower))
            .cloned()
            .collect()
    }

    pub fn get_relevant_for_ai(&self, context: &HashMap<String, String>, limit: usize) -> Vec<HistoryEntry> {
        let guard = match self.storage.read() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };

        let mut scored: Vec<(f64, HistoryEntry)> = guard.entries.iter().cloned().filter_map(|entry| {
            let mut score = 0.0;

            for (key, value) in context {
                if entry.command.contains(key.as_str()) {
                    score += 1.0;
                }
                if entry.command.contains(value.as_str()) {
                    score += 1.5;
                }
            }

            if entry.exit_code == 0 {
                score += 0.5;
            }

            let recent_bonus = Self::recency_score(entry.timestamp);
            score += recent_bonus;

            if score > 0.0 {
                Some((score, entry))
            } else {
                None
            }
        }).collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().map(|(_, entry)| entry).take(limit).collect()
    }

    fn recency_score(timestamp: u64) -> f64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let age_seconds = now.saturating_sub(timestamp);
        let hour = 3600u64;
        if age_seconds < hour {
            2.0
        } else if age_seconds < hour * 24 {
            1.0
        } else if age_seconds < hour * 24 * 7 {
            0.5
        } else {
            0.1
        }
    }

    pub fn clear(&self) -> Result<(), String> {
        let mut guard = self.storage.write().map_err(|e| format!("Lock error: {}", e))?;
        guard.entries.clear();
        let json = serde_json::to_string_pretty(&*guard)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;
        fs::write(&self.history_path, json)
            .map_err(|e| format!("Failed to write history file: {}", e))?;
        Ok(())
    }

    pub fn len(&self) -> usize {
        match self.storage.read() {
            Ok(g) => g.entries.len(),
            Err(_) => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_entry_creation() {
        let entry = HistoryEntry::new("ls -la");
        assert_eq!(entry.command, "ls -la");
        assert_eq!(entry.exit_code, -1);
        assert!(entry.ai_suggestion.is_none());
    }

    #[test]
    fn test_history_entry_with_ai() {
        let entry = HistoryEntry::new("ls -la").with_ai_suggestion("ls -lah");
        assert_eq!(entry.ai_suggestion, Some("ls -lah".to_string()));
    }

    #[test]
    fn test_mark_completed() {
        let mut entry = HistoryEntry::new("ls -la");
        entry.mark_completed(0, 150);
        assert_eq!(entry.exit_code, 0);
        assert_eq!(entry.duration_ms, 150);
    }

    #[test]
    fn test_add_and_get_recent() {
        let tmp = std::env::temp_dir().join("shellai_test_history.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));

        hm.add_entry(HistoryEntry::new("cmd1")).unwrap();
        hm.add_entry(HistoryEntry::new("cmd2")).unwrap();
        hm.add_entry(HistoryEntry::new("cmd3")).unwrap();

        let recent = hm.get_recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].command, "cmd2");
        assert_eq!(recent[1].command, "cmd3");

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_search() {
        let tmp = std::env::temp_dir().join("shellai_test_history2.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));

        hm.add_entry(HistoryEntry::new("git status")).unwrap();
        hm.add_entry(HistoryEntry::new("ls -la")).unwrap();
        hm.add_entry(HistoryEntry::new("git log")).unwrap();

        let results = hm.search("git");
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| e.command.contains("git")));

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_save_and_load() {
        let tmp = std::env::temp_dir().join("shellai_test_history3.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));

        hm.add_entry(HistoryEntry::new("echo hello")).unwrap();
        hm.save().unwrap();

        let hm2 = HistoryManager::new(Some(tmp.to_str().unwrap()));
        let loaded = hm2.load().unwrap();
        assert_eq!(loaded, 1);
        assert_eq!(hm2.get_recent(1)[0].command, "echo hello");

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_max_entries_enforced() {
        let tmp = std::env::temp_dir().join("shellai_test_history4.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap())).with_max_entries(3);

        for i in 0..5 {
            hm.add_entry(HistoryEntry::new(&format!("cmd{}", i))).unwrap();
        }

        assert_eq!(hm.len(), 3);
        let entries = hm.get_recent(3);
        assert_eq!(entries[0].command, "cmd2");
        assert_eq!(entries[2].command, "cmd4");

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_clear() {
        let tmp = std::env::temp_dir().join("shellai_test_history5.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));

        hm.add_entry(HistoryEntry::new("cmd1")).unwrap();
        hm.clear().unwrap();
        assert!(hm.is_empty());

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_relevant_for_ai() {
        let tmp = std::env::temp_dir().join("shellai_test_history6.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));

        hm.add_entry(HistoryEntry::new("git status")).unwrap();
        hm.add_entry(HistoryEntry::new("ls -la")).unwrap();
        hm.add_entry(HistoryEntry::new("cd /home")).unwrap();

        let mut ctx = HashMap::new();
        ctx.insert("PWD".to_string(), "/home".to_string());
        ctx.insert("GIT_BRANCH".to_string(), "main".to_string());

        let relevant = hm.get_relevant_for_ai(&ctx, 2);
        assert!(!relevant.is_empty());
        assert!(relevant.len() <= 2);

        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let tmp = std::env::temp_dir().join("shellai_test_noexist.json");
        let hm = HistoryManager::new(Some(tmp.to_str().unwrap()));
        let count = hm.load().unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_recency_score() {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert_eq!(HistoryManager::recency_score(now), 2.0);
        assert_eq!(HistoryManager::recency_score(now - 1800), 2.0);
        assert_eq!(HistoryManager::recency_score(now - 40000), 1.0);
        assert_eq!(HistoryManager::recency_score(now - 86400 * 3), 0.5);
        assert_eq!(HistoryManager::recency_score(0), 0.1);
    }
}
