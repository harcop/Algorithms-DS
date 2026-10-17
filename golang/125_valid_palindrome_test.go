package leetcode

import "testing"

func TestIsPalindromeStringExampleOne(t *testing.T) {
	if !isPalindromeString("A man, a plan, a canal: Panama") {
		t.Fatal("isPalindromeString() = false, want true")
	}
}

func TestIsPalindromeStringExampleTwo(t *testing.T) {
	if isPalindromeString("race a car") {
		t.Fatal("isPalindromeString() = true, want false")
	}
}

func TestIsPalindromeStringExampleThree(t *testing.T) {
	if !isPalindromeString(" ") {
		t.Fatal("isPalindromeString() = false, want true")
	}
}
