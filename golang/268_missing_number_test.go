package leetcode

import "testing"

func TestMissingNumberExampleOne(t *testing.T) {
	if got := missingNumber([]int{3, 0, 1}); got != 2 {
		t.Fatalf("missingNumber() = %d, want 2", got)
	}
}

func TestMissingNumberExampleTwo(t *testing.T) {
	if got := missingNumber([]int{0, 1}); got != 2 {
		t.Fatalf("missingNumber() = %d, want 2", got)
	}
}

func TestMissingNumberExampleThree(t *testing.T) {
	if got := missingNumber([]int{9, 6, 4, 2, 3, 5, 7, 0, 1}); got != 8 {
		t.Fatalf("missingNumber() = %d, want 8", got)
	}
}
