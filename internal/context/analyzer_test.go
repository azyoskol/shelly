package context

import (
	"os"
	"path/filepath"
	"testing"
)

func TestAnalyzeContextReturnsWorkingDirectory(t *testing.T) {
	// Create temp directory with known file
	tmpDir, err := os.MkdirTemp("", "shelly-context-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	testFile := filepath.Join(tmpDir, "test.sh")
	if err := os.WriteFile(testFile, []byte("#!/bin/bash\necho hello\n"), 0755); err != nil {
		t.Fatalf("failed to create test file: %v", err)
	}

	// Capture environment that includes current working directory
	env := make(map[string]any)
	env["PWD"] = tmpDir

	got, err := AnalyzeContext(env)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if got.WorkingDirectory != tmpDir {
		t.Errorf("expected WorkingDirectory=%s, got %s", tmpDir, got.WorkingDirectory)
	}
}