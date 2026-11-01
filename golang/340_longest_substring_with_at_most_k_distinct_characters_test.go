package leetcode

import "testing"

func TestLengthOfLongestSubstringKDistinctExamples(t *testing.T) {
	if got := lengthOfLongestSubstringKDistinct("eceba", 2); got != 3 {
		t.Fatalf("lengthOfLongestSubstringKDistinct() = %d, want 3", got)
	}
	if got := lengthOfLongestSubstringKDistinct("aa", 1); got != 2 {
		t.Fatalf("lengthOfLongestSubstringKDistinct() = %d, want 2", got)
	}
}
