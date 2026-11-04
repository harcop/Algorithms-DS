package leetcode

import "testing"

func TestLongestSubstringExamples(t *testing.T) {
	if got := longestSubstring("aaabb", 3); got != 3 {
		t.Fatalf("longestSubstring() = %d, want 3", got)
	}
	if got := longestSubstring("ababbc", 2); got != 5 {
		t.Fatalf("longestSubstring() = %d, want 5", got)
	}
}
