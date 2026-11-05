package leetcode

// LeetCode #419 - Battleships in a Board
//
// A ship is counted at the cell that has no ship directly above it or to its left.
func countBattleships(board [][]byte) int {
	ships := 0
	for r := range board {
		for c := range board[r] {
			if board[r][c] != 'X' {
				continue
			}
			if r > 0 && board[r-1][c] == 'X' {
				continue
			}
			if c > 0 && board[r][c-1] == 'X' {
				continue
			}
			ships++
		}
	}
	return ships
}
