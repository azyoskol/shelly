package shell

import (
	"github.com/anomalyco/opencode/tools/shelly/pkg/shelly"
)

type FishAdapter struct {
	callback func() []shelly.Suggestion
}

func NewFishCompletionAdapter(callback func() []shelly.Suggestion) *FishAdapter {
	return &FishAdapter{callback: callback}
}

func (f *FishAdapter) GetSuggestions() []shelly.Suggestion {
	if f.callback != nil {
		return f.callback()
	}
	return []shelly.Suggestion{}
}