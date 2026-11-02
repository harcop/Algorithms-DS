package leetcode

// LeetCode #361 - Bomb Enemy
//
// Reuse the enemy count along a row or column until a wall starts a new open segment.
func maxKilledEnemies(grid [][]byte) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	best := 0
	rowHits := 0
	colHits := make([]int, cols)
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if c == 0 || grid[r][c-1] == 'W' {
				rowHits = 0
				for k := c; k < cols && grid[r][k] != 'W'; k++ {
					if grid[r][k] == 'E' {
						rowHits++
					}
				}
			}
			if r == 0 || grid[r-1][c] == 'W' {
				colHits[c] = 0
				for k := r; k < rows && grid[k][c] != 'W'; k++ {
					if grid[k][c] == 'E' {
						colHits[c]++
					}
				}
			}
			if grid[r][c] == '0' && rowHits+colHits[c] > best {
				best = rowHits + colHits[c]
			}
		}
	}
	return best
}
