package leetcode

import "testing"

func TestThreeSumSmallerExampleOne(t *testing.T) {
	if got := threeSumSmaller([]int{-2, 0, 1, 3}, 2); got != 2 {
		t.Fatalf("threeSumSmaller() = %d, want 2", got)
	}
}

func TestThreeSumSmallerExampleTwo(t *testing.T) {
	if got := threeSumSmaller(nil, 0); got != 0 {
		t.Fatalf("threeSumSmaller() = %d, want 0", got)
	}
}
