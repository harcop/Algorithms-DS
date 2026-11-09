package leetcode

import "testing"

func TestMinMoves2Example(t *testing.T) {
	if got := minMoves2([]int{1, 2, 3}); got != 2 {
		t.Fatalf("minMoves2() = %d, want 2", got)
	}
	if got := minMoves2([]int{1, 10, 2, 9}); got != 16 {
		t.Fatalf("minMoves2() = %d, want 16", got)
	}
}
