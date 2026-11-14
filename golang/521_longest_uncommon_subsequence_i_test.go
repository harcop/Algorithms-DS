package leetcode

import "testing"

func TestFindLUSlengthExamples(t *testing.T) {
	if got := findLUSlength("aba", "cdc"); got != 3 {
		t.Fatalf("findLUSlength() = %d, want 3", got)
	}
	if got := findLUSlength("aaa", "aaa"); got != -1 {
		t.Fatalf("findLUSlength() = %d, want -1", got)
	}
}
