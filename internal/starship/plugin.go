package starship

import (
	"encoding/json"
)

type Plugin struct{}

func NewPlugin() *Plugin {
	return &Plugin{}
}

func (p *Plugin) GenerateModuleInfo() string {
	info := map[string]interface{}{
		"name":    "shelly",
		"version": "1.0.0",
		"help":    `Shelly provides context-aware shell command suggestions using AI-powered completions.`,
	}

	bytes, _ := json.Marshal(info)
	return string(bytes)
}