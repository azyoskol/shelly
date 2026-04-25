package shelly

import (
	"context"
	"errors"
	"testing"

	"github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

func TestGetSuggestionsReturnsErrContextUnavailableWithEmptyInput(t *testing.T) {
	req := shelly.CompletionRequest{
		Input:   "",
		Context: map[string]any{"cwd": "/"},
		History: []string{},
		Settings: shelly.SuggestionSettings{},
	}

	_, err := shelly.GetSuggestions(context.Background(), req)
	if !errors.Is(err, shelly.ErrContextUnavailable) {
		t.Errorf("expected ErrContextUnavailable, got %v", err)
	}
}