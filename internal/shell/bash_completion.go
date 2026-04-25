package shell

import (
	"github.com/azyoskol/shelly/pkg/shelly"
)

type BashAdapter struct {
	callback func() []shelly.Suggestion
}

func NewBashCompletionAdapter(callback func() []shelly.Suggestion) *BashAdapter {
	return &BashAdapter{callback: callback}
}

func (b *BashAdapter) GetSuggestions() []shelly.Suggestion {
	if b.callback != nil {
		return b.callback()
	}
	return []shelly.Suggestion{}
}