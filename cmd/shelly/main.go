package main

import (
	"context"
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"github.com/anomalyco/opencode/tools/shelly/internal/agents"
	"github.com/anomalyco/opencode/tools/shelly/internal/shell"
	"github.com/anomalyco/opencode/tools/shelly/internal/starship"
	"github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

var rootCmd = &cobra.Command{
	Use:   "shelly",
	Short: "Intelligent shell completion assistant",
}

var pluginCmd = &cobra.Command{
	Use:   "plugin",
	Short: "Generate starship plugin metadata",
	Run: func(cmd *cobra.Command, args []string) {
		plugin := starship.NewPlugin()
		fmt.Println(plugin.GenerateModuleInfo())
	},
}

func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		os.Exit(1)
	}
}

func init() {
	rootCmd.AddCommand(pluginCmd)

	// Initialize standard commands agent
	stdAgent := agents.NewStandardCommandsAgent()
	agents.InitCommonCommands(stdAgent)

	// Register default agent in registry
	reg := agents.NewRegistry()
	reg.Register("default", stdAgent)

	// Create shell adapters with context-aware suggestion callback
	_ = shell.NewBashCompletionAdapter(func() []shelly.Suggestion {
		ctx, cancel := context.WithTimeout(context.Background(), 500)
		defer cancel()

		req := shelly.CompletionRequest{
			Input:   "git",
			Context: make(map[string]any),
			History: []string{},
			Settings: shelly.SuggestionSettings{
				EnableSessionHistory:    true,
				SessionMaxSize:          100,
				IncludeEnvironmentInfo: true,
			},
		}

		suggestions, err := shelly.GetSuggestions(ctx, req)
		if err != nil {
			fmt.Fprintf(os.Stderr, "warning: failed to get suggestions: %v\n", err)
			return []shelly.Suggestion{}
		}

		// Sort by priority and limit to 50 results
		type indexedSuggestion struct {
			shelly.Suggestion
			Priority int
		}
		indexed := make([]indexedSuggestion, len(suggestions))
		for i, s := range suggestions {
			indexed[i] = indexedSuggestion{Suggestion: s, Priority: s.Priority}
		}

		// Sort descending by priority
		for i := 0; i < len(indexed)-1; i++ {
			for j := 0; j < len(indexed)-i-1; j++ {
				if indexed[j].Priority < indexed[j+1].Priority {
					indexed[j], indexed[j+1] = indexed[j+1], indexed[j]
				}
			}
		}

		if len(indexed) > 50 {
			indexed = indexed[:50]
		}

		result := make([]shelly.Suggestion, len(indexed))
		for i, item := range indexed {
			result[i] = item.Suggestion
		}

		return result
	})

	// Initialize other adapters
	_ = shell.NewZshCompletionAdapter(nil)
	_ = shell.NewFishCompletionAdapter(nil)
}

func main() {
	Execute()
}