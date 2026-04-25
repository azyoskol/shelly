package shell

import (
	"github.com/azyoskol/shelly/pkg/shelly"
)

type ZshAdapter struct {
	callback func() []shelly.Suggestion
}

func NewZshCompletionAdapter(callback func() []shelly.Suggestion) *ZshAdapter {
	return &ZshAdapter{callback: callback}
}

func (z *ZshAdapter) GetSuggestions() []shelly.Suggestion {
	if z.callback != nil {
		return z.callback()
	}
	return []shelly.Suggestion{}
}