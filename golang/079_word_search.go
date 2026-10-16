package leetcode

// LeetCode #79 - Word Search
//
// Depth-first search from each cell, marking the current cell so it cannot be reused.
func exist(board [][]byte, word string) bool {
	if len(board) == 0 || len(board[0]) == 0 {
		return false
	}
	m, n := len(board), len(board[0])

	var dfs func(i, j, k int) bool
	dfs = func(i, j, k int) bool {
		if k == len(word) {
			return true
		}
		if i < 0 || j < 0 || i >= m || j >= n || board[i][j] != word[k] {
			return false
		}
		saved := board[i][j]
		board[i][j] = '#'
		ok := dfs(i-1, j, k+1) || dfs(i+1, j, k+1) || dfs(i, j-1, k+1) || dfs(i, j+1, k+1)
		board[i][j] = saved
		return ok
	}

	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if dfs(i, j, 0) {
				return true
			}
		}
	}
	return false
}
