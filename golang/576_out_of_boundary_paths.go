package leetcode

// LeetCode #576 - Out of Boundary Paths
//
// Each move either steps off the grid or lands on a neighbor, and the ways add up modulo 10^9+7.
func findPaths(m int, n int, maxMove int, startRow int, startColumn int) int {
	const mod = 1_000_000_007
	ways := make([][]int, m)
	for row := range ways {
		ways[row] = make([]int, n)
	}
	ways[startRow][startColumn] = 1
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	out := 0
	for move := 0; move < maxMove; move++ {
		next := make([][]int, m)
		for row := range next {
			next[row] = make([]int, n)
		}
		for row := 0; row < m; row++ {
			for col := 0; col < n; col++ {
				if ways[row][col] == 0 {
					continue
				}
				for _, dir := range dirs {
					nextRow, nextCol := row+dir[0], col+dir[1]
					if nextRow < 0 || nextCol < 0 || nextRow >= m || nextCol >= n {
						out = (out + ways[row][col]) % mod
						continue
					}
					next[nextRow][nextCol] = (next[nextRow][nextCol] + ways[row][col]) % mod
				}
			}
		}
		ways = next
	}
	return out
}
