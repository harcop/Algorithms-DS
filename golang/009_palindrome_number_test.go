package leetcode

import "testing"

func TestIsPalindromeExampleOne(t *testing.T) {
	if !isPalindrome(121) {
		t.Fatal("isPalindrome(121) = false, want true")
	}
}

func TestIsPalindromeExampleTwo(t *testing.T) {
	if isPalindrome(-121) {
		t.Fatal("isPalindrome(-121) = true, want false")
	}
}

func TestIsPalindromeExampleThree(t *testing.T) {
	if isPalindrome(10) {
		t.Fatal("isPalindrome(10) = true, want false")
	}
}

func TestIsPalindromeZero(t *testing.T) {
	if !isPalindrome(0) {
		t.Fatal("isPalindrome(0) = false, want true")
	}
}
