package leetcode

import "testing"

func TestSolveExampleOne(t *testing.T) {
	board := [][]byte{
		[]byte("XXXX"),
		[]byte("XOOX"),
		[]byte("XXOX"),
		[]byte("XOXX"),
	}
	solve(board)
	want := [][]byte{
		[]byte("XXXX"),
		[]byte("XXXX"),
		[]byte("XXXX"),
		[]byte("XOXX"),
	}
	for r := range want {
		if !sameBytes(board[r], want[r]) {
			t.Fatalf("solve() row %d = %s, want %s", r, board[r], want[r])
		}
	}
}
