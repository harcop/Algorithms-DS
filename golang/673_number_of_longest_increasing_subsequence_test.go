package leetcode

import "testing"

func TestFindNumberOfLISExamples(t *testing.T) {
	if got := findNumberOfLIS([]int{1, 3, 5, 4, 7}); got != 2 {
		t.Fatalf("findNumberOfLIS() = %d, want 2", got)
	}
	if got := findNumberOfLIS([]int{2, 2, 2, 2, 2}); got != 5 {
		t.Fatalf("findNumberOfLIS() = %d, want 5", got)
	}
}
