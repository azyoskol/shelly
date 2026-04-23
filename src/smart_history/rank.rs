/// Search result with ranking score
#[derive(Debug, Clone)]
pub struct SearchRank {
    /// The matched command
    pub command: String,
    /// Working directory
    pub cwd: String,
    /// Relevance score (higher = better match)
    pub score: f64,
    /// How many times this command was run
    pub frequency: u32,
}

impl SearchRank {
    /// Create a new search result
    pub fn new(command: String, cwd: String, score: f64, frequency: u32) -> Self {
        SearchRank {
            command,
            cwd,
            score,
            frequency,
        }
    }
}
