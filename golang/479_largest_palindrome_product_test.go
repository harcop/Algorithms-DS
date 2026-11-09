package leetcode

import "testing"

func TestLargestPalindromeExamples(t *testing.T) {
	if got := largestPalindrome(2); got != 987 {
		t.Fatalf("largestPalindrome(2) = %d, want 987", got)
	}
	if got := largestPalindrome(1); got != 9 {
		t.Fatalf("largestPalindrome(1) = %d, want 9", got)
	}
}
