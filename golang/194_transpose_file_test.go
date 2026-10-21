package leetcode

import (
	"strings"
	"testing"
)

func TestTransposeFileScript(t *testing.T) {
	if !strings.Contains(transposeFileScript, "awk") {
		t.Fatalf("transposeFileScript = %q, want awk", transposeFileScript)
	}
}
