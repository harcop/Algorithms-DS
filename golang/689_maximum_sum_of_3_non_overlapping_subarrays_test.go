package leetcode

import "testing"

func TestMaxSumOfThreeSubarraysExample(t *testing.T) {
	got := maxSumOfThreeSubarrays([]int{1, 2, 1, 2, 6, 7, 5, 1}, 2)
	if !sameInts(got, []int{0, 3, 5}) {
		t.Fatalf("maxSumOfThreeSubarrays() = %v, want [0 3 5]", got)
	}
}
