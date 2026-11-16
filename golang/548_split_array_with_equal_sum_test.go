package leetcode

import "testing"

func TestSplitArrayEqualSumExamples(t *testing.T) {
	if !splitArrayEqualSum([]int{1, 2, 1, 2, 1, 2, 1}) {
		t.Fatal("splitArrayEqualSum([1 2 1 2 1 2 1]) = false, want true")
	}
	if splitArrayEqualSum([]int{1, 2, 1, 2, 1, 2, 1, 2}) {
		t.Fatal("splitArrayEqualSum([1 2 1 2 1 2 1 2]) = true, want false")
	}
}
