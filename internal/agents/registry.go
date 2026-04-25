package agents

import (
	"strings"
)

type Agent interface {
	GenerateSuggestions(input string) ([]Suggestion, error)
}

type Registry struct {
	agents map[string]Agent
}

func NewRegistry() *Registry {
	return &Registry{agents: make(map[string]Agent)}
}

func (r *Registry) Register(name string, agent Agent) {
	r.agents[name] = agent
}

func (r *Registry) GetSuggestions(input string) ([]Suggestion, error) {
	// Default to standard commands if no specific match found
	if agent, exists := r.agents["default"]; exists && input != "" {
		return agent.GenerateSuggestions(input)
	}

	// Try other registered agents based on input prefix
	for name, agent := range r.agents {
		if shouldUseAgent(name, input) {
			suggestions, err := agent.GenerateSuggestions(input)
			if err == nil && len(suggestions) > 0 {
				return suggestions, nil
			}
		}
	}

	// Fallback to standard commands
	stdAgent := NewStandardCommandsAgent()
	InitCommonCommands(stdAgent)
	return stdAgent.GenerateSuggestions(input)
}

func shouldUseAgent(agentName string, input string) bool {
	switch agentName {
	case "github":
		return len(input) > 0 && (strings.Contains(strings.ToLower(input), "repo") ||
			strings.Contains(strings.ToLower(input), "clone") ||
			strings.Contains(strings.ToLower(input), "org"))
	case "localfiles":
		return true
	default:
		return false
	}
}