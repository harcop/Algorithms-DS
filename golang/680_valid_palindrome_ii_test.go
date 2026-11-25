package leetcode

import "testing"

func TestValidPalindromeExamples(t *testing.T) {
	if !validPalindrome("aba") {
		t.Fatal("validPalindrome(aba) = false, want true")
	}
	if !validPalindrome("abca") {
		t.Fatal("validPalindrome(abca) = false, want true")
	}
	if validPalindrome("abc") {
		t.Fatal("validPalindrome(abc) = true, want false")
	}
}
