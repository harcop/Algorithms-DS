package leetcode

import "testing"

func TestLongestPalindromeLengthExamples(t *testing.T) {
	if got := longestPalindromeLength("abccccdd"); got != 7 {
		t.Fatalf("longestPalindromeLength() = %d, want 7", got)
	}
	if got := longestPalindromeLength("a"); got != 1 {
		t.Fatalf("longestPalindromeLength() = %d, want 1", got)
	}
}
