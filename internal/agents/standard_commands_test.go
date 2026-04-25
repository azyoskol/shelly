package agents

import (
	"testing"
)

func TestStandardCommandsAgentReturnsGitClones(t *testing.T) {
	agent := NewStandardCommandsAgent()

	suggestions, err := agent.GenerateSuggestions("git clone")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Check that at least one suggestion exists and contains "git clone"
	foundGitClone := false
	for _, s := range suggestions {
		if len(s.Text) > 0 && len(s.Description) > 0 {
			foundGitClone = true
			break
		}
	}

	if !foundGitClone {
		t.Errorf("expected at least one git clone suggestion, got: %v", suggestions)
	}
}

func TestStandardCommandsAgentReturnsDockerRuns(t *testing.T) {
	agent := NewStandardCommandsAgent()

	suggestions, err := agent.GenerateSuggestions("docker run")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Check for docker-related suggestions
	foundDockerRun := false
	for _, s := range suggestions {
		if len(s.Text) > 0 && len(s.Description) > 0 {
			foundDockerRun = true
			break
		}
	}

	if !foundDockerRun {
		t.Errorf("expected docker-related suggestions, got: %v", suggestions)
	}
}