package context

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
)

type Context struct {
	WorkingDirectory    string              `json:"working_directory"`
	ActiveEnvironments   []EnvironmentInfo  `json:"active_environments"`
	CommandHistory       []string            `json:"command_history,omitempty"`
	FilesInDir            []FileInfo          `json:"files_in_dir,omitempty"`
}

type EnvironmentInfo struct {
	Type        string // "conda", "virtualenv", "nvm"
	Name        string
	Executable  string
}

type FileInfo struct {
	Name      string
	IsExec    bool
	SizeBytes int64
}

var ErrContextUnavailable = errors.New("context analysis failed")

func AnalyzeContext(env map[string]any) (Context, error) {
	ctx := Context{
		WorkingDirectory:  "/",
		ActiveEnvironments: []EnvironmentInfo{},
	}

	// Extract PWD from environment or get actual working directory
	if pwd, ok := env["PWD"]; ok {
		pwdStr, isString := pwd.(string)
		if !isString {
			return ctx, ErrContextUnavailable
		}
		ctx.WorkingDirectory = filepath.Clean(pwdStr)
	} else {
		cwd, err := os.Getwd()
		if err != nil {
			// Fallback to current process working directory
			cwd, _ = filepath.Abs(".")
			ctx.WorkingDirectory = cwd
		} else {
			ctx.WorkingDirectory = cwd
		}
	}

	// Detect active environments by scanning PATH for common prefixes
	detectEnvironments(&ctx)

	return ctx, nil
}

func detectEnvironments(ctx *Context) {
	env := os.Environ()
	_ = env
	// Check for common environment prefixes in PATH
	pathEnv := os.Getenv("PATH")
	if pathEnv != "" {
		paths := strings.Split(pathEnv, ":")
		for _, p := range paths {
			if strings.Contains(p, "/opt/conda/") {
				ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
					Type:      "conda",
					Name:      filepath.Base(p),
					Executable: p + "/bin/activate",
				})
			} else if strings.Contains(p, "/.local/share/virtualenvs/") {
				ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
					Type:      "virtualenv",
					Name:      filepath.Base(p),
					Executable: p + "/bin/activate",
				})
			} else if strings.Contains(p, "/nvm/") {
				ctx.ActiveEnvironments = append(ctx.ActiveEnvironments, EnvironmentInfo{
					Type:      "nvm",
					Name:      filepath.Base(filepath.Dir(p)),
					Executable: p + "/node",
				})
			}
		}
	}
}

func containsPrefix(dir, prefix string) bool {
	return strings.HasPrefix(dir, prefix) || strings.Contains(dir, "/"+prefix+"/")
}