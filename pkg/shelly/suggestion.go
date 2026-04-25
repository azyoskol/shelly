package shelly

import (
	"context"

	ctxanalyzer "github.com/anomalyco/opencode/tools/shelly/internal/context"
)

type Suggestion struct {
	Text        string `json:"text"`
	Description string `json:"description,omitempty"`
	Priority    int    `json:"priority,omitempty"`
}

type SuggestionSettings struct {
	EnableSessionHistory    bool `json:"enable_session_history"`
	SessionMaxSize          int  `json:"session_max_size"`
	IncludeEnvironmentInfo bool `json:"include_environment_info"`
}

type CompletionRequest struct {
	Input     string               // Partial command being typed
	Context  map[string]any      // Working directory, env vars, etc.
	History  []string             // Recent commands from session
	Settings SuggestionSettings  // From config.yaml
}

func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
	if req.Input == "" {
		return nil, ErrContextUnavailable
	}

	// Analyze context and enrich request
	analyzedCtx, err := ctxanalyzer.AnalyzeContext(req.Context)
	if err != nil {
		return nil, err
	}

	_ = analyzedCtx

	// Here we would call the appropriate agent based on input
	// For now, returning empty as stub
	return []Suggestion{}, nil
}