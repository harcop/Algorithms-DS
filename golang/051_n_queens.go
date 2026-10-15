package leetcode

// LeetCode #51 - N-Queens
//
// Place one queen per row, rejecting any column or diagonal already used.
func solveNQueens(n int) [][]string {
	board := make([][]byte, n)
	for i := range board {
		board[i] = make([]byte, n)
		for j := range board[i] {
			board[i][j] = '.'
		}
	}
	cols := make([]bool, n)
	diag1 := make([]bool, 2*n-1)
	diag2 := make([]bool, 2*n-1)
	var result [][]string

	var dfs func(row int)
	dfs = func(row int) {
		if row == n {
			solution := make([]string, n)
			for i := range board {
				solution[i] = string(board[i])
			}
			result = append(result, solution)
			return
		}
		for col := 0; col < n; col++ {
			d1 := row + col
			d2 := row + (n - 1 - col)
			if cols[col] || diag1[d1] || diag2[d2] {
				continue
			}
			cols[col], diag1[d1], diag2[d2] = true, true, true
			board[row][col] = 'Q'
			dfs(row + 1)
			board[row][col] = '.'
			cols[col], diag1[d1], diag2[d2] = false, false, false
		}
	}

	dfs(0)
	return result
}
