package leetcode

import "testing"

func TestNumSquaresExampleOne(t *testing.T) {
	if got := numSquares(12); got != 3 {
		t.Fatalf("numSquares() = %d, want 3", got)
	}
}

func TestNumSquaresExampleTwo(t *testing.T) {
	if got := numSquares(13); got != 2 {
		t.Fatalf("numSquares() = %d, want 2", got)
	}
}
