package leetcode

// LeetCode #289 - Game of Life
//
// A live cell stays live with two or three neighbors, and a dead cell comes alive with three.
func gameOfLife(board [][]int) {
	m, n := len(board), len(board[0])
	orig := make([][]int, m)
	for i := range board {
		orig[i] = append([]int(nil), board[i]...)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			live := 0
			for di := -1; di <= 1; di++ {
				for dj := -1; dj <= 1; dj++ {
					if di == 0 && dj == 0 {
						continue
					}
					row, col := i+di, j+dj
					if row >= 0 && col >= 0 && row < m && col < n {
						live += orig[row][col]
					}
				}
			}
			if orig[i][j] == 1 {
				if live == 2 || live == 3 {
					board[i][j] = 1
				} else {
					board[i][j] = 0
				}
			} else if live == 3 {
				board[i][j] = 1
			} else {
				board[i][j] = 0
			}
		}
	}
}
