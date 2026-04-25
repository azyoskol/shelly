# Shelly - Context-Aware Typo Correction & History Learning

**Date:** 2026-04-25  
**Version:** 1.0.0  
**Status:** Approved for Implementation  

## 1. Overview

Shelly provides intelligent, context-aware command completions with typo correction and AI-powered history learning. This design implements Level 1 fuzzy matching (edit-distance ≤ 1) with configurable privacy controls, historical pattern analysis, and a unified ranking system that combines multiple scoring sources.

### Key Features
- **Typo Correction**: Automatic suggestions for common typos with visual indicators
- **History Learning**: Pattern-based learning from successful command sequences
- **Privacy Controls**: Granular configuration via `config.yaml` with local/cloud modes
- **Unified Ranking**: Combined scoring system prioritizing exact matches, then fuzzy corrections, then historical patterns

## 2. Architecture

### 2.1 High-Level Diagram

```
┌─────────────┐     ┌───────────┐     ┌───────────┐
│   User Input │──▶│ Completion │──▶│  Ranking  │
│    (e.g.,   │     │  Engine   │     │  System   │
│  "git clon")│     │            │     │          │
└─────────────┘     └───────────┘     └───────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
    ┌─────────────┐   ┌─────────────┐   ┌─────────────┐
    │  Typo Engine│   │History Analyzer│   │Context Analyzer│
    └─────────────┘   └─────────────┘   └─────────────┘
```

### 2.2 Component Breakdown

#### 2.2.1 Completion Request Flow
1. User types partial command (e.g., `git clon`)
2. ContextAnalyzer extracts working directory, environment, recent history
3. TypoEngine generates fuzzy matches with edit-distance ≤ 1
4. HistoryAnalyzer provides historical pattern suggestions
5. RankingSystem combines scores from all sources
6. Results filtered and deduplicated for display

#### 2.2.2 Scoring Algorithm

```go
// Base score (exact match)
if exactMatch:
    baseScore = 100

// Fuzzy score (typo matching)
fuzzyScore = fuzzyDistance * 50  // Lower distance = higher score

// Historical boost
historyBoost = historicalSimilarity * 30

// Final ranking
finalScore = baseScore + fuzzyScore + historyBoost
```

#### 2.2.3 Privacy Configuration Model
All privacy settings controlled via `config.yaml`:
- `typo_correction.enabled`: Enable/disable typo correction entirely
- `history.enable_local_history_learning`: Local pattern learning
- `history.privacy_mode`: "local" (default) or "cloud" (anonymized patterns)
- `history.retention_days`: Data retention period

## 3. Component Design & Implementation Details

### 3.1 Typo Engine Interface

```go
// internal/typo/engine.go
type TypoEngine interface {
    GetFuzzyMatches(input string, maxDistance int) []MatchResult
}

type MatchResult struct {
    Text              string
    Distance           int
    IsTypoCorrection   bool  // true if this is a known typo correction
    SuggestionType     string  // "command" | "file" | "path"
}
```

**Implementation Details:**
- Uses Levenshtein distance for edit-distance calculation
- Pre-computed dictionary of common typos (git clone → git clon, etc.)
- Context-aware filtering based on working directory
- Input sanitization to prevent injection attacks
- Rate limiting to prevent memory exhaustion

### 3.2 History Analyzer Interface

```go
// internal/history/analyzer.go
type HistoryAnalyzer interface {
    GetHistoricalSuggestions(input string, context Context) []HistoryMatch
}

type HistoryMatch struct {
    Suggestion      string
    Frequency       int           // How often this command was run successfully
    Relevance       float64       // Based on working directory and recent usage
    LastUsed        time.Time     // When last used in similar context
}
```

**Implementation Details:**
- Session history from config (enable_session_history)
- Working directory context for relevance scoring
- Frequency-based weighting with decay over time
- Privacy-first data handling: only local storage by default
- Anonymized pattern aggregation for cloud mode

### 3.3 Ranking System Interface

```go
// internal/ranking/system.go
type RankingSystem struct {
    typoEngine     TypoEngine
    historyAnalyzer HistoryAnalyzer
    config         CompletionConfig
}

func (r *RankingSystem) RankSuggestions(input string, context Context) []RankedSuggestion {
    // Combine scores with configurable weights
    // Apply privacy filters based on config settings
}

type RankedSuggestion struct {
    Suggestion       shelly.Suggestion
    Score            float64
    Source           string  // "exact", "typo", "history"
    IsTypoCorrection bool
}
```

**Implementation Details:**
- Normalized scores to prevent extreme values
- Tie-breaking with secondary sorting criteria (alphabetical)
- Fallback to standard completions when no matches found
- Caching strategies for efficiency (LRU cache with TTL)

## 4. Error Handling & Edge Cases

### 4.1 Typo Correction Edge Cases

```go
// Prevent infinite loops with recursive typos
if len(normalizedInput) > 50 {
    return nil  // Too long for fuzzy matching
}

// Limit results to prevent overwhelming UI
maxResults := 100
if len(matches) > maxResults {
    matches = matches[:maxResults]
}
```

### 4.2 History Analyzer Edge Cases

```go
// Privacy-first data handling
if !config.EnableLocalHistoryLearning {
    return nil
}

// Sanitize input before processing
sanitizedInput := strings.ReplaceAll(input, `"`, "")
sanitizedInput = regexp.MustCompile(`[<>|&;]`).ReplaceAllString(sanitizedInput, " ")
```

### 4.3 Ranking System Edge Cases

```go
// Prevent score overflow/underflow
normalizedScores := normalizeScores(scores, minScore=0, maxScore=100)

// Handle ties gracefully with secondary sorting criteria
sort.Slice(rankings, func(i, j int) bool {
    if rankings[i].Score == rankings[j].Score {
        return rankings[i].Source < rankings[j].Source  // Alphabetical tiebreaker
    }
    return rankings[i].Score > rankings[j].Score
})
```

### 4.4 Performance Considerations

```go
// Caching strategies for efficiency
type Cache struct {
    typoCache   *lru.Cache      // Input → Matches (TTL: 5 minutes)
    historyCache *lru.Cache     // Context → Suggestions (TTL: 10 minutes)
}

func NewCache() *Cache {
    return &Cache{
        typoCache:   lru.New(1000),      // Limit cache size to prevent memory issues
        historyCache: lru.New(500),       // Smaller cache for historical data
    }
}
```

## 5. Testing Strategy

### 5.1 Unit Tests

```go
// internal/typo/engine_test.go
func TestTypoEngine_GetFuzzyMatches(t *testing.T) {
    // Test exact match (distance = 0)
    matches := typoEngine.GetFuzzyMatches("git clone", 1)
    assert.Contains(t, matches, "git clone")
    
    // Test Level 1 fuzzy matching (edit-distance ≤ 1)
    matches = typoEngine.GetFuzzyMatches("git clon", 1)
    assert.Contains(t, matches, "git clone")
    assert.Equal(t, 1, matches[0].Distance)
    
    // Test no match beyond edit-distance limit
    matches = typoEngine.GetFuzzyMatches("git clonx", 1)
    assert.Empty(t, matches)
}
```

### 5.2 Integration Tests

```go
// tests/integration/typo_integration_test.go
func TestTypoIntegrationWithGitCommands(t *testing.T) {
    // Setup test environment with git context
    ctx := Context{
        WorkingDir: "/home/user/project",
        CommandHistory: []string{"git status", "git add ."},
    }
    
    req := CompletionRequest{
        Input: "git clon",
        Context: ctx,
        Settings: SuggestionSettings{
            EnableSessionHistory: true,
        },
    }
    
    suggestions, err := GetSuggestions(context.Background(), req)
    assert.NoError(t, err)
    assert.NotEmpty(t, suggestions)
}
```

### 5.3 Performance Benchmarks

```go
// benchmarks/typo_benchmark_test.go
func BenchmarkTypoEngine_GetFuzzyMatches(b *testing.B) {
    for i := 0; i < b.N; i++ {
        typoEngine.GetFuzzyMatches("git clon", 1)
    }
}

// Expected performance targets:
// - Typo engine: <5ms per query (P95)
// - History analyzer: <10ms per query (P95)
// - Ranking system: <20ms total processing time
```

### 5.4 Security Testing

```go
// tests/security/typo_security_test.go
func TestTypoEngine_SanitizesInput(t *testing.T) {
    // Ensure special characters are sanitized before processing
    matches := typoEngine.GetFuzzyMatches("git clon; rm -rf /", 1)
    assert.Empty(t, matches)  // Should reject potentially dangerous input
}
```

## 6. Configuration Schema

### 6.1 Updated `config.yaml` Schema

```yaml
completion:
  # Typo correction settings
  typo_correction:
    enabled: true                    # Enable/disable typo correction entirely (default: true)
    max_edit_distance: 1             # Maximum edit distance for fuzzy matching (Level 1: ≤1, Level 2: ≤2) (default: 1)
    show_typo_indicator: true        # Show ✓ indicator for typo corrections (default: true)
    visual_style: "subtle"           # Styling options: "none", "subtle", "prominent" (default: "subtle")

  # History learning settings
  history:
    enable_local_history_learning: true  # Enable local command pattern learning (default: true)
    privacy_mode: "local"                 # Privacy mode: "local" or "cloud" (anonymized patterns) (default: "local")
    retention_days: 30                    # How long to retain history data (default: 30)

# Context analyzer settings
context:
  enable_session_history: true           # Store previous interactions for better suggestions (default: true)
  session_max_size: 100                  # Maximum entries in session history (default: 100)
  include_environment_info: true         # Include active conda/virtualenv info (default: true)

# LLM provider settings (for advanced features)
llm:
  provider: "openai"                     # Options: openai, anthropic, huggingface, ollama, lmstudio
  model: "gpt-4o"
  max_tokens: 2048
  temperature: 0.7
  timeout_seconds: 30

# Rate limiting for LLM queries
rate_limiting:
  requests_per_second: 10
  burst_size: 20

# Fallback behavior when LLM unavailable
fallback:
  use_cached_results: true
  cache_ttl_seconds: 3600
```

### 6.2 Documentation Updates for README.md

```markdown
## Typo Correction & History Learning

Shelly intelligently handles typos and learns from your command patterns while respecting your privacy preferences.

### Typo Correction
- **Level 1 Fuzzy Matching**: Automatically suggests corrections for common typos (e.g., `git clon` → `git clone`)
- **Configurable Behavior**: Enable/disable via config, adjust visual indicators
- **Context-Aware**: Prioritizes suggestions based on your working directory and recent commands

### History Learning
- **Local Pattern Analysis**: Learns from successful command sequences without sending data anywhere
- **Privacy First**: All learning happens locally by default; cloud mode is opt-in with anonymized patterns only
- **Session Continuity**: Maintains context across sessions for better suggestions over time

### Privacy Controls
All privacy settings are configurable in `config.yaml`:

```yaml
completion:
  typo_correction:
    enabled: true
    show_typo_indicator: true
    visual_style: "subtle"

history:
  enable_local_history_learning: true
  privacy_mode: "local"
  retention_days: 30
```

See [Configuration Guide](./docs/config-guide.md) for detailed options and examples.
```

## 7. Success Criteria

1. **Zero Friction Setup**: Seamless integration with bash/zsh/fish completions
2. **Context Awareness**: Accurate and relevant command suggestions based on current context
3. **Performance Targets**:
   - < 5ms latency for typo engine (P95)
   - < 10ms latency for history analyzer (P95)
   - < 20ms total processing time for ranking system
4. **Backward Compatibility**: No breaking changes to existing shell workflows
5. **Configurability**: All behavior controlled via `config.yaml` with sensible defaults
6. **Privacy First**: Local learning by default, opt-in cloud mode with anonymized patterns only

## 8. Security Considerations

- Never store API keys in repository; use environment variables or secure storage
- Validate all external inputs from shell adapters to prevent injection attacks
- Implement TLS for all external HTTP connections including Ollama/LMStudio
- Sanitize and escape all user input before displaying in terminal output
- Rate limit history analysis to prevent memory exhaustion
- Privacy-first data handling: only local storage by default

## 9. Versioning

Follow Semantic Versioning (SemVer):
- MAJOR: Breaking changes to CLI interface or shell adapter behavior
- MINOR: New features, backwards-compatible improvements
- PATCH: Bug fixes and non-breaking changes

## 10. Contribution Guidelines

See `CONTRIBUTING.md` for coding standards, PR process, and code review guidelines.

## 11. License

MIT License - see `LICENSE` file for details.
