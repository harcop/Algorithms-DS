package leetcode

import "testing"

func TestPathSumIVExamples(t *testing.T) {
	if got := pathSumIV([]int{113, 215, 221}); got != 12 {
		t.Fatalf("pathSumIV() = %d, want 12", got)
	}
	if got := pathSumIV([]int{113, 221}); got != 4 {
		t.Fatalf("pathSumIV() = %d, want 4", got)
	}
}
