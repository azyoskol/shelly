package shelly

import (
	"context"

	ctxanalyzer "github.com/azyoskol/shelly/internal/context"
	"github.com/azyoskol/shelly/internal/agents"
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

	// Use registry to get suggestions based on input
	reg := agents.NewRegistry()
	agent := agents.NewStandardCommandsAgent()
	agents.InitCommonCommands(agent)
	reg.Register("default", agent)

	suggestions, err := reg.GetSuggestions(req.Input)
	if err != nil {
		return nil, err
	}

	// Convert from agents.Suggestion to shelly.Suggestion
	result := make([]Suggestion, len(suggestions))
	for i, s := range suggestions {
		result[i] = Suggestion{
			Text:        s.Text,
			Description: s.Description,
			Priority:    s.Priority,
		}
	}

	return result, nil
}