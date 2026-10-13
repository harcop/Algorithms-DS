package leetcode

import "testing"

func TestLongestCommonPrefixExampleOne(t *testing.T) {
	got := longestCommonPrefix([]string{"flower", "flow", "flight"})
	if got != "fl" {
		t.Fatalf("longestCommonPrefix() = %q, want %q", got, "fl")
	}
}

func TestLongestCommonPrefixExampleTwo(t *testing.T) {
	got := longestCommonPrefix([]string{"dog", "racecar", "car"})
	if got != "" {
		t.Fatalf("longestCommonPrefix() = %q, want empty", got)
	}
}
