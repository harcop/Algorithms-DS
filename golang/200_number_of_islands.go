package leetcode

// LeetCode #200 - Number of Islands
//
// Each unvisited land cell starts an island; flood fill marks the rest of it.
func numIslands(grid [][]byte) int {
	m := len(grid)
	if m == 0 {
		return 0
	}
	n := len(grid[0])
	var flood func(i, j int)
	flood = func(i, j int) {
		if i < 0 || j < 0 || i >= m || j >= n || grid[i][j] != '1' {
			return
		}
		grid[i][j] = '0'
		flood(i-1, j)
		flood(i+1, j)
		flood(i, j-1)
		flood(i, j+1)
	}
	count := 0
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if grid[i][j] == '1' {
				count++
				flood(i, j)
			}
		}
	}
	return count
}
