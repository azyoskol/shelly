//! Smart History module for Shally
//! Provides intelligent command history with fuzzy search, ranking, and context awareness

mod entry;
mod history;
mod rank;

pub use entry::HistoryEntry;
pub use history::SmartHistory;
pub use rank::SearchRank;
