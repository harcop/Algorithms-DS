package leetcode

// LeetCode #562 - Longest Line of Consecutive One in Matrix
//
// Each cell of a line extends the run arriving from the previous cell in that direction.
func longestLine(mat [][]int) int {
	if len(mat) == 0 || len(mat[0]) == 0 {
		return 0
	}
	rows, cols := len(mat), len(mat[0])
	horizontal := make([][]int, rows)
	vertical := make([][]int, rows)
	diagonal := make([][]int, rows)
	anti := make([][]int, rows)
	for row := range horizontal {
		horizontal[row] = make([]int, cols)
		vertical[row] = make([]int, cols)
		diagonal[row] = make([]int, cols)
		anti[row] = make([]int, cols)
	}
	best := 0
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			if mat[row][col] == 0 {
				continue
			}
			horizontal[row][col] = 1
			vertical[row][col] = 1
			diagonal[row][col] = 1
			anti[row][col] = 1
			if col > 0 {
				horizontal[row][col] += horizontal[row][col-1]
			}
			if row > 0 {
				vertical[row][col] += vertical[row-1][col]
			}
			if row > 0 && col > 0 {
				diagonal[row][col] += diagonal[row-1][col-1]
			}
			if row > 0 && col+1 < cols {
				anti[row][col] += anti[row-1][col+1]
			}
			best = max(best, horizontal[row][col], vertical[row][col], diagonal[row][col], anti[row][col])
		}
	}
	return best
}
