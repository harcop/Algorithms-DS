package leetcode

import "testing"

func TestTotalNQueensExampleOne(t *testing.T) {
	got := totalNQueens(4)
	if got != 2 {
		t.Fatalf("totalNQueens() = %d, want 2", got)
	}
}

func TestTotalNQueensExampleTwo(t *testing.T) {
	got := totalNQueens(1)
	if got != 1 {
		t.Fatalf("totalNQueens() = %d, want 1", got)
	}
}
