package leetcode

import (
	"strings"
	"testing"
)

func TestTenthLineScript(t *testing.T) {
	if !strings.Contains(tenthLineScript, "10") {
		t.Fatalf("tenthLineScript = %q, want line 10", tenthLineScript)
	}
}
