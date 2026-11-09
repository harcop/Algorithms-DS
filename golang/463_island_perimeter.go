package leetcode

// LeetCode #463 - Island Perimeter
//
// Each land cell starts with four sides, and every shared edge with land above or to the left removes two sides.
func islandPerimeter(grid [][]int) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	perimeter := 0
	for i := range grid {
		for j := range grid[i] {
			if grid[i][j] == 0 {
				continue
			}
			perimeter += 4
			if i > 0 && grid[i-1][j] == 1 {
				perimeter -= 2
			}
			if j > 0 && grid[i][j-1] == 1 {
				perimeter -= 2
			}
		}
	}
	return perimeter
}
