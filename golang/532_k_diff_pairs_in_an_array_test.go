package leetcode

import "testing"

func TestFindPairsExamples(t *testing.T) {
	if got := findPairs([]int{3, 1, 4, 1, 5}, 2); got != 2 {
		t.Fatalf("findPairs() = %d, want 2", got)
	}
	if got := findPairs([]int{1, 2, 3, 4, 5}, 1); got != 4 {
		t.Fatalf("findPairs() = %d, want 4", got)
	}
	if got := findPairs([]int{1, 3, 1, 5, 4}, 0); got != 1 {
		t.Fatalf("findPairs() = %d, want 1", got)
	}
}
