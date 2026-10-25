package leetcode

import "testing"

func TestIsPalindromeListExampleOne(t *testing.T) {
	if !isPalindromeList(vecToList([]int{1, 2, 2, 1})) {
		t.Fatal("isPalindromeList() = false, want true")
	}
}

func TestIsPalindromeListExampleTwo(t *testing.T) {
	if isPalindromeList(vecToList([]int{1, 2})) {
		t.Fatal("isPalindromeList() = true, want false")
	}
}
