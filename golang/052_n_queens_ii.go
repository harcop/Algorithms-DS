package leetcode

// LeetCode #52 - N-Queens II
//
// Count queen placements by marking used columns and both diagonals.
func totalNQueens(n int) int {
	cols := make([]bool, n)
	diag1 := make([]bool, 2*n-1)
	diag2 := make([]bool, 2*n-1)

	var dfs func(row int) int
	dfs = func(row int) int {
		if row == n {
			return 1
		}
		count := 0
		for col := 0; col < n; col++ {
			d1 := row + col
			d2 := row + (n - 1 - col)
			if cols[col] || diag1[d1] || diag2[d2] {
				continue
			}
			cols[col], diag1[d1], diag2[d2] = true, true, true
			count += dfs(row + 1)
			cols[col], diag1[d1], diag2[d2] = false, false, false
		}
		return count
	}

	return dfs(0)
}
