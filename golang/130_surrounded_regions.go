package leetcode

// LeetCode #130 - Surrounded Regions
//
// Mark every O that touches the border, then flip the unmarked O cells to X.
func solve(board [][]byte) {
	if len(board) == 0 || len(board[0]) == 0 {
		return
	}
	m, n := len(board), len(board[0])

	var mark func(i, j int)
	mark = func(i, j int) {
		if i < 0 || j < 0 || i >= m || j >= n || board[i][j] != 'O' {
			return
		}
		board[i][j] = '#'
		mark(i-1, j)
		mark(i+1, j)
		mark(i, j-1)
		mark(i, j+1)
	}

	for i := 0; i < m; i++ {
		mark(i, 0)
		mark(i, n-1)
	}
	for j := 0; j < n; j++ {
		mark(0, j)
		mark(m-1, j)
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			switch board[i][j] {
			case '#':
				board[i][j] = 'O'
			case 'O':
				board[i][j] = 'X'
			}
		}
	}
}
