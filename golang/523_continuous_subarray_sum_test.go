package leetcode

import "testing"

func TestCheckSubarraySumExamples(t *testing.T) {
	if !checkSubarraySum([]int{23, 2, 4, 6, 7}, 6) {
		t.Fatal("checkSubarraySum([23,2,4,6,7], 6) = false, want true")
	}
	if !checkSubarraySum([]int{23, 2, 6, 4, 7}, 6) {
		t.Fatal("checkSubarraySum([23,2,6,4,7], 6) = false, want true")
	}
	if checkSubarraySum([]int{23, 2, 6, 4, 7}, 13) {
		t.Fatal("checkSubarraySum([23,2,6,4,7], 13) = true, want false")
	}
}
