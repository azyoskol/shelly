package agents

import (
	"strings"
)

type Suggestion struct {
	Text        string
	Description string
	Priority    int
}

type StandardCommandsAgent struct {
	cache *commandCache
}

type commandCache struct {
	entries map[string][]string
}

func NewStandardCommandsAgent() *StandardCommandsAgent {
	return &StandardCommandsAgent{
		cache: &commandCache{entries: make(map[string][]string)},
	}
}

func (agent *StandardCommandsAgent) GenerateSuggestions(input string) ([]Suggestion, error) {
	if input == "" {
		return nil, nil
	}

	lowerInput := strings.ToLower(strings.TrimSpace(input))

	// Initialize commands if not already done
	if len(agent.cache.entries) == 0 {
		InitCommonCommands(agent)
	}

	// Check for command-specific suggestions
	var suggestions []string

	switch {
	case strings.HasPrefix(lowerInput, "git"):
		suggestions = agent.cache.entries["git"]
	case strings.HasPrefix(lowerInput, "docker") || strings.Contains(lowerInput, "container") || strings.Contains(lowerInput, "podman"):
		suggestions = agent.cache.entries["docker"]
	case strings.HasPrefix(lowerInput, "go"):
		suggestions = agent.cache.entries["go"]
	default:
		// Fallback to git commands for common prefix
		suggestions = agent.cache.entries["git"]

		// Check if input looks like a command prefix we should suggest completions for
		commonCommands := map[string][]string{
			"c": {"cd ..", "cp -r src/", "cat README.md"},
			"e": {"echo 'hello world'", "export PATH=$PATH:~/bin"},
			"v": {"vim README.md", "view .env.local", "version -all"},
		}

		if len(lowerInput) > 0 {
			if cmd, ok := commonCommands[lowerInput[:1]]; ok {
				suggestions = append(suggestions, cmd...)
			}
		}
	}

	// Deduplicate and limit results
	seen := make(map[string]bool)
	uniqueSuggestions := []string{}
	for _, s := range suggestions {
		if !seen[s] && len(uniqueSuggestions) < 10 {
			seen[s] = true
			uniqueSuggestions = append(uniqueSuggestions, s)
		}
	}

	// Convert to Suggestion format
	result := make([]Suggestion, len(uniqueSuggestions))
	for i, s := range uniqueSuggestions {
		result[i] = Suggestion{
			Text:       s,
			Description: extractDescription(s),
			Priority:   10 - (i % 5),
		}
	}

	return result, nil
}

func InitCommonCommands(agent *StandardCommandsAgent) {
	// Git commands
	agent.cache.entries["git"] = []string{
		"git clone https://github.com/azyoskol/shelly.git",
		"git checkout -b feature/ai-completions",
		"git push origin main",
		"git pull upstream main",
		"git diff HEAD~1",
	}

	// Docker commands
	agent.cache.entries["docker"] = []string{
		"docker run -it --rm shelly:latest",
		"docker build . -t shelly",
		"docker compose up -d",
		"docker exec -it shelly_container bash",
		"docker logs shelly -f",
	}

	// Go commands
	agent.cache.entries["go"] = []string{
		"go mod init github.com/azyoskol/shelly",
		"go get github.com/spf13/cobra@latest",
		"go test ./... -coverprofile=coverage.out",
		"go run cmd/shelly/main.go --help",
		"go vet ./...",
	}
}

func extractDescription(suggestion string) string {
	parts := strings.SplitN(suggestion, " ", 2)
	if len(parts) < 2 {
		return ""
	}

	cmd := parts[0]
	desc := parts[1]
	if len(desc) > 64 {
		desc = desc[:64]
	}

	switch cmd {
	case "git":
		return desc + ", git command"
	case "docker":
		return desc + ", docker container command"
	case "go":
		return desc + ", go toolchain command"
	default:
		return desc
	}
}