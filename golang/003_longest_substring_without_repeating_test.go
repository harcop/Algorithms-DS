package leetcode

import "testing"

func TestLengthOfLongestSubstringExampleOne(t *testing.T) {
	if got := lengthOfLongestSubstring("abcabcbb"); got != 3 {
		t.Fatalf("lengthOfLongestSubstring() = %d, want 3", got)
	}
}

func TestLengthOfLongestSubstringExampleTwo(t *testing.T) {
	if got := lengthOfLongestSubstring("bbbbb"); got != 1 {
		t.Fatalf("lengthOfLongestSubstring() = %d, want 1", got)
	}
}

func TestLengthOfLongestSubstringExampleThree(t *testing.T) {
	if got := lengthOfLongestSubstring("pwwkew"); got != 3 {
		t.Fatalf("lengthOfLongestSubstring() = %d, want 3", got)
	}
}
