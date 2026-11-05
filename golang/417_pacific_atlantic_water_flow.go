package leetcode

import "math"

// LeetCode #417 - Pacific Atlantic Water Flow
//
// Water reaches an ocean by flowing uphill from that ocean's shore, so a cell in both searches drains to both.
func pacificAtlantic(heights [][]int) [][]int {
	rows := len(heights)
	if rows == 0 || len(heights[0]) == 0 {
		return nil
	}
	cols := len(heights[0])
	pacific := make([][]bool, rows)
	atlantic := make([][]bool, rows)
	for r := range heights {
		pacific[r] = make([]bool, cols)
		atlantic[r] = make([]bool, cols)
	}
	var flow func(seen [][]bool, r, c, prev int)
	flow = func(seen [][]bool, r, c, prev int) {
		if r < 0 || c < 0 || r >= rows || c >= cols || seen[r][c] || heights[r][c] < prev {
			return
		}
		seen[r][c] = true
		height := heights[r][c]
		flow(seen, r-1, c, height)
		flow(seen, r+1, c, height)
		flow(seen, r, c-1, height)
		flow(seen, r, c+1, height)
	}
	for c := 0; c < cols; c++ {
		flow(pacific, 0, c, math.MinInt)
		flow(atlantic, rows-1, c, math.MinInt)
	}
	for r := 0; r < rows; r++ {
		flow(pacific, r, 0, math.MinInt)
		flow(atlantic, r, cols-1, math.MinInt)
	}
	var out [][]int
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if pacific[r][c] && atlantic[r][c] {
				out = append(out, []int{r, c})
			}
		}
	}
	return out
}
