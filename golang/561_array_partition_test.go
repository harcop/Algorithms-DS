package leetcode

import "testing"

func TestArrayPairSumExamples(t *testing.T) {
	if got := arrayPairSum([]int{1, 4, 3, 2}); got != 4 {
		t.Fatalf("arrayPairSum = %d, want 4", got)
	}
	if got := arrayPairSum([]int{6, 2, 6, 5, 1, 2}); got != 9 {
		t.Fatalf("arrayPairSum = %d, want 9", got)
	}
}
