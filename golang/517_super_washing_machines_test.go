package leetcode

import "testing"

func TestFindMinMovesExamples(t *testing.T) {
	if got := findMinMoves([]int{1, 0, 5}); got != 3 {
		t.Fatalf("findMinMoves([1 0 5]) = %d, want 3", got)
	}
	if got := findMinMoves([]int{0, 3, 0}); got != 2 {
		t.Fatalf("findMinMoves([0 3 0]) = %d, want 2", got)
	}
	if got := findMinMoves([]int{0, 2, 0}); got != -1 {
		t.Fatalf("findMinMoves([0 2 0]) = %d, want -1", got)
	}
}
