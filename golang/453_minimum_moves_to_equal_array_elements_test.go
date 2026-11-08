package leetcode

import "testing"

func TestMinMovesExample(t *testing.T) {
	if got := minMoves([]int{1, 2, 3}); got != 3 {
		t.Fatalf("minMoves() = %d, want 3", got)
	}
}
