package shelly

import (
	"context"
	"testing"

	"github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

func TestGetSuggestionsReturnsEmptyWhenNoInput(t *testing.T) {
	req := shelly.CompletionRequest{
		Input:   "",
		Context: map[string]any{"cwd": "/"},
		History: []string{},
		Settings: shelly.SuggestionSettings{
			EnableSessionHistory: true,
			SessionMaxSize:       100,
		},
	}

	got, err := shelly.GetSuggestions(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(got) != 0 {
		t.Errorf("expected empty suggestions, got %d items", len(got))
	}
}