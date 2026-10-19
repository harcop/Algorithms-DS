package leetcode

import "testing"

func TestLengthOfLongestSubstringTwoDistinctExampleOne(t *testing.T) {
	got := lengthOfLongestSubstringTwoDistinct("eceba")
	if got != 3 {
		t.Fatalf("lengthOfLongestSubstringTwoDistinct() = %d, want 3", got)
	}
}

func TestLengthOfLongestSubstringTwoDistinctExampleTwo(t *testing.T) {
	got := lengthOfLongestSubstringTwoDistinct("ccaabbb")
	if got != 5 {
		t.Fatalf("lengthOfLongestSubstringTwoDistinct() = %d, want 5", got)
	}
}
