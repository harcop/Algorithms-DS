package leetcode

import "testing"

func TestCanPermutePalindromeExampleOne(t *testing.T) {
	if !canPermutePalindrome("carerac") {
		t.Fatal("canPermutePalindrome() = false, want true")
	}
}

func TestCanPermutePalindromeExampleTwo(t *testing.T) {
	if canPermutePalindrome("code") {
		t.Fatal("canPermutePalindrome() = true, want false")
	}
}
