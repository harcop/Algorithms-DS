package leetcode

import "testing"

func TestFourSumCountExample(t *testing.T) {
	got := fourSumCount([]int{1, 2}, []int{-2, -1}, []int{-1, 2}, []int{0, 2})
	if got != 2 {
		t.Fatalf("fourSumCount() = %d, want 2", got)
	}
}
