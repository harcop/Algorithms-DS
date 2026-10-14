package leetcode

// LeetCode #36 - Valid Sudoku
//
// Track which digits already appear in each row, column, and 3x3 box.
func isValidSudoku(board [][]byte) bool {
	var rows, cols, boxes [9][9]bool

	for r := 0; r < 9; r++ {
		for c := 0; c < 9; c++ {
			ch := board[r][c]
			if ch == '.' {
				continue
			}
			digit := ch - '1'
			box := (r/3)*3 + c/3
			if rows[r][digit] || cols[c][digit] || boxes[box][digit] {
				return false
			}
			rows[r][digit] = true
			cols[c][digit] = true
			boxes[box][digit] = true
		}
	}

	return true
}
