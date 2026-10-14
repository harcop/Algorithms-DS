package leetcode

import "testing"

func TestIsValidSudokuExampleOne(t *testing.T) {
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
	if !isValidSudoku(board) {
		t.Fatal("isValidSudoku() = false, want true")
	}
}

func TestIsValidSudokuExampleTwo(t *testing.T) {
	board := sudokuBoard(
		"83..7....",
		"6..195...",
		".98....6.",
		"8...6...3",
		"4..8.3..1",
		"7...2...6",
		".6....28.",
		"...419..5",
		"....8..79",
	)
	if isValidSudoku(board) {
		t.Fatal("isValidSudoku() = true, want false")
	}
}

func sudokuBoard(rows ...string) [][]byte {
	board := make([][]byte, len(rows))
	for i, row := range rows {
		board[i] = []byte(row)
	}
	return board
}
