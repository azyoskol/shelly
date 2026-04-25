package shelly

import "context"

type Suggestion struct {
	Text        string `json:"text"`
	Description string `json:"description,omitempty"`
	Priority    int    `json:"priority,omitempty"`
}

type CompletionRequest struct {
	Input     string               // Partial command being typed
	Context  map[string]any      // Working directory, env vars, etc.
	History  []string             // Recent commands from session
	Settings SuggestionSettings  // From config.yaml
}

type SuggestionSettings struct {
	EnableSessionHistory    bool `json:"enable_session_history"`
	SessionMaxSize          int  `json:"session_max_size"`
	IncludeEnvironmentInfo bool `json:"include_environment_info"`
}

func GetSuggestions(ctx context.Context, req CompletionRequest) ([]Suggestion, error) {
	if req.Input == "" {
		return nil, ErrContextUnavailable
	}
	return []Suggestion{}, nil
}