package leetcode

import "testing"

func TestSolveNQueensExampleOne(t *testing.T) {
	got := solveNQueens(4)
	if len(got) != 2 {
		t.Fatalf("solveNQueens() = %v, want 2 solutions", got)
	}
}

func TestSolveNQueensExampleTwo(t *testing.T) {
	got := solveNQueens(1)
	want := [][]string{{"Q"}}
	if len(got) != 1 || len(got[0]) != 1 || got[0][0] != want[0][0] {
		t.Fatalf("solveNQueens() = %v, want %v", got, want)
	}
}
