package leetcode

import "testing"

func TestVerifyPreorderExampleOne(t *testing.T) {
	if !verifyPreorder([]int{5, 2, 1, 3, 6}) {
		t.Fatal("verifyPreorder() = false, want true")
	}
}

func TestVerifyPreorderExampleTwo(t *testing.T) {
	if verifyPreorder([]int{5, 2, 6, 3, 1}) {
		t.Fatal("verifyPreorder() = true, want false")
	}
}
