package leetcode

import "testing"

func TestSolveSudokuExample(t *testing.T) {
	board := sudokuBoard(
		"53..7....",
		"6..195...",
		".98....6.",
		"8...6...3",
		"4..8.3..1",
		"7...2...6",
		".6....28.",
		"...419..5",
		"....8..79",
	)
	want := sudokuBoard(
		"534678912",
		"672195348",
		"198342567",
		"859761423",
		"426853791",
		"713924856",
		"961537284",
		"287419635",
		"345286179",
	)

	solveSudoku(board)
	for r := 0; r < 9; r++ {
		if !sameBytes(board[r], want[r]) {
			t.Fatalf("solveSudoku() row %d = %s, want %s", r, board[r], want[r])
		}
	}
}

func sameBytes(a, b []byte) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
