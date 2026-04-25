package llm

import (
	"context"
)

type Suggestion struct {
	Text        string
	Description string
	Priority    int
}

type Client interface {
	GenerateSuggestions(ctx context.Context, input string, settings map[string]any) ([]Suggestion, error)
}

type StubClient struct{}

func NewStubClient() *StubClient {
	return &StubClient{}
}

func (c *StubClient) GenerateSuggestions(ctx context.Context, input string, settings map[string]any) ([]Suggestion, error) {
	return []Suggestion{}, nil
}