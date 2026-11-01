package leetcode

// LeetCode #329 - Longest Increasing Path in a Matrix
//
// Each cell remembers the longest strictly increasing path that starts there.
func longestIncreasingPath(matrix [][]int) int {
	rows := len(matrix)
	if rows == 0 {
		return 0
	}
	cols := len(matrix[0])
	memo := make([][]int, rows)
	for i := range memo {
		memo[i] = make([]int, cols)
	}
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	var walk func(row, col int) int
	walk = func(row, col int) int {
		if memo[row][col] > 0 {
			return memo[row][col]
		}
		best := 1
		for _, dir := range dirs {
			nextRow, nextCol := row+dir[0], col+dir[1]
			if nextRow < 0 || nextCol < 0 || nextRow >= rows || nextCol >= cols {
				continue
			}
			if matrix[nextRow][nextCol] > matrix[row][col] {
				if length := 1 + walk(nextRow, nextCol); length > best {
					best = length
				}
			}
		}
		memo[row][col] = best
		return best
	}
	best := 0
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			if length := walk(row, col); length > best {
				best = length
			}
		}
	}
	return best
}
