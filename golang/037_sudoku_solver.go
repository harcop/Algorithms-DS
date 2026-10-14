package leetcode

// LeetCode #37 - Sudoku Solver
//
// Fill empty cells in order, placing a digit only when its row, column, and box allow it.
func solveSudoku(board [][]byte) {
	var dfs func(cell int) bool
	dfs = func(cell int) bool {
		if cell == 81 {
			return true
		}
		row := cell / 9
		col := cell % 9
		if board[row][col] != '.' {
			return dfs(cell + 1)
		}
		for digit := byte('1'); digit <= '9'; digit++ {
			if sudokuCellOK(board, row, col, digit) {
				board[row][col] = digit
				if dfs(cell + 1) {
					return true
				}
				board[row][col] = '.'
			}
		}
		return false
	}
	dfs(0)
}

func sudokuCellOK(board [][]byte, row, col int, digit byte) bool {
	for c := 0; c < 9; c++ {
		if board[row][c] == digit {
			return false
		}
	}
	for r := 0; r < 9; r++ {
		if board[r][col] == digit {
			return false
		}
	}
	boxRow := 3 * (row / 3)
	boxCol := 3 * (col / 3)
	for r := 0; r < 3; r++ {
		for c := 0; c < 3; c++ {
			if board[boxRow+r][boxCol+c] == digit {
				return false
			}
		}
	}
	return true
}
