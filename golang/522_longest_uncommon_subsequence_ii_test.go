package leetcode

import "testing"

func TestFindLUSlengthIIExamples(t *testing.T) {
	if got := findLUSlengthII([]string{"aba", "cdc", "eae"}); got != 3 {
		t.Fatalf("findLUSlengthII() = %d, want 3", got)
	}
	if got := findLUSlengthII([]string{"aaa", "aaa", "aa"}); got != -1 {
		t.Fatalf("findLUSlengthII() = %d, want -1", got)
	}
}
