package leetcode

import "testing"

func TestReversePairsExamples(t *testing.T) {
	if got := reversePairs([]int{1, 3, 2, 3, 1}); got != 2 {
		t.Fatalf("reversePairs() = %d, want 2", got)
	}
	if got := reversePairs([]int{2, 4, 3, 5, 1}); got != 3 {
		t.Fatalf("reversePairs() = %d, want 3", got)
	}
}
