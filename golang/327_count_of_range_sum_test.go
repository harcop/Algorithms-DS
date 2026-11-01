package leetcode

import "testing"

func TestCountRangeSumExample(t *testing.T) {
	if got := countRangeSum([]int{-2, 5, -1}, -2, 2); got != 3 {
		t.Fatalf("countRangeSum() = %d, want 3", got)
	}
}
