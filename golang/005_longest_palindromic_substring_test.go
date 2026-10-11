package leetcode

import "testing"

func TestLongestPalindromeExampleOne(t *testing.T) {
	out := longestPalindrome("babad")
	if out != "bab" && out != "aba" {
		t.Fatalf("longestPalindrome() = %q, want \"bab\" or \"aba\"", out)
	}
}

func TestLongestPalindromeExampleTwo(t *testing.T) {
	if got := longestPalindrome("cbbd"); got != "bb" {
		t.Fatalf("longestPalindrome() = %q, want \"bb\"", got)
	}
}
