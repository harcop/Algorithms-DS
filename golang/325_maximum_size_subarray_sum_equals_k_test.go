package leetcode

import "testing"

func TestMaxSubArrayLenExamples(t *testing.T) {
	if got := maxSubArrayLen([]int{1, -1, 5, -2, 3}, 3); got != 4 {
		t.Fatalf("maxSubArrayLen() = %d, want 4", got)
	}
	if got := maxSubArrayLen([]int{-2, -1, 2, 1}, 1); got != 2 {
		t.Fatalf("maxSubArrayLen() = %d, want 2", got)
	}
}
