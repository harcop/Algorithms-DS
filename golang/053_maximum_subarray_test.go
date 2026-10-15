package leetcode

import "testing"

func TestMaxSubArrayExampleOne(t *testing.T) {
	got := maxSubArray([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4})
	if got != 6 {
		t.Fatalf("maxSubArray() = %d, want 6", got)
	}
}

func TestMaxSubArrayExampleTwo(t *testing.T) {
	got := maxSubArray([]int{1})
	if got != 1 {
		t.Fatalf("maxSubArray() = %d, want 1", got)
	}
}

func TestMaxSubArrayExampleThree(t *testing.T) {
	got := maxSubArray([]int{5, 4, -1, 7, 8})
	if got != 23 {
		t.Fatalf("maxSubArray() = %d, want 23", got)
	}
}
