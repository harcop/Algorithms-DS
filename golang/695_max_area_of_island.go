package leetcode

// LeetCode #695 - Max Area of Island
//
// A land cell is cleared as it is counted, so each island is measured once.
func maxAreaOfIsland(grid [][]int) int {
	var area func(r, c int) int
	area = func(r, c int) int {
		if r < 0 || c < 0 || r >= len(grid) || c >= len(grid[0]) || grid[r][c] == 0 {
			return 0
		}
		grid[r][c] = 0
		return 1 + area(r+1, c) + area(r-1, c) + area(r, c+1) + area(r, c-1)
	}
	best := 0
	for r := range grid {
		for c := range grid[r] {
			best = max(best, area(r, c))
		}
	}
	return best
}
