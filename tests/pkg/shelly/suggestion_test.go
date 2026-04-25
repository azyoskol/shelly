package shelly

import (
	"context"
	"errors"
	"testing"

	"github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

func TestGetSuggestionsReturnsEmptySliceWithEmptyInput(t *testing.T) {
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
	if err == nil {
		t.Fatalf("expected error, got nil")
	}
	
	if !errors.Is(err, shelly.ErrContextUnavailable) {
		t.Errorf("expected ErrContextUnavailable, got %v", err)
	}

	if len(got) != 0 {
		t.Errorf("expected empty suggestions, got %d items", len(got))
	}
}