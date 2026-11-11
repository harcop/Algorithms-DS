package leetcode

import "testing"

func TestFindTargetSumWaysExamples(t *testing.T) {
	if got := findTargetSumWays([]int{1, 1, 1, 1, 1}, 3); got != 5 {
		t.Fatalf("findTargetSumWays() = %d, want 5", got)
	}
	if got := findTargetSumWays([]int{1}, 1); got != 1 {
		t.Fatalf("findTargetSumWays() = %d, want 1", got)
	}
}
