package leetcode

import "testing"

func TestSubarraySumExample(t *testing.T) {
	if got := subarraySum([]int{1, 1, 1}, 2); got != 2 {
		t.Fatalf("subarraySum = %d, want 2", got)
	}
}
