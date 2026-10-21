package leetcode

import (
	"strings"
	"testing"
)

func TestWordFrequencyScript(t *testing.T) {
	if !strings.Contains(wordFrequencyScript, "uniq") {
		t.Fatalf("wordFrequencyScript = %q, want uniq", wordFrequencyScript)
	}
}
