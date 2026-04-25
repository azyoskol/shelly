package shelly

import "errors"

var ErrLLMTimeout = errors.New("llm request timed out")
var ErrContextUnavailable = errors.New("context analysis failed")
var ErrShellAdapterNotRegistered = errors.New("shell adapter not registered")