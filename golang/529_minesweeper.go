package leetcode

// LeetCode #529 - Minesweeper
//
// A mine click is marked, and an empty click reveals a blank region or the count of neighboring mines.
func updateBoard(board [][]byte, click []int) [][]byte {
	row, col := click[0], click[1]
	if board[row][col] == 'M' {
		board[row][col] = 'X'
		return board
	}
	var reveal func(r, c int)
	reveal = func(r, c int) {
		if r < 0 || c < 0 || r >= len(board) || c >= len(board[0]) || board[r][c] != 'E' {
			return
		}
		mines := neighboringMines(board, r, c)
		if mines > 0 {
			board[r][c] = '0' + mines
			return
		}
		board[r][c] = 'B'
		for dr := -1; dr <= 1; dr++ {
			for dc := -1; dc <= 1; dc++ {
				if dr == 0 && dc == 0 {
					continue
				}
				reveal(r+dr, c+dc)
			}
		}
	}
	reveal(row, col)
	return board
}

func neighboringMines(board [][]byte, row, col int) byte {
	mines := byte(0)
	for dr := -1; dr <= 1; dr++ {
		for dc := -1; dc <= 1; dc++ {
			if dr == 0 && dc == 0 {
				continue
			}
			r, c := row+dr, col+dc
			if r < 0 || c < 0 || r >= len(board) || c >= len(board[0]) {
				continue
			}
			if board[r][c] == 'M' {
				mines++
			}
		}
	}
	return mines
}
