package leetcode

import "testing"

func TestShortestPalindromeExampleOne(t *testing.T) {
	got := shortestPalindrome("aacecaaa")
	if got != "aaacecaaa" {
		t.Fatalf("shortestPalindrome() = %q, want %q", got, "aaacecaaa")
	}
}

func TestShortestPalindromeExampleTwo(t *testing.T) {
	got := shortestPalindrome("abcd")
	if got != "dcbabcd" {
		t.Fatalf("shortestPalindrome() = %q, want %q", got, "dcbabcd")
	}
}
