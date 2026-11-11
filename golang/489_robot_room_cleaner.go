package leetcode

// LeetCode #489 - Robot Room Cleaner
//
// Every open cell reachable from the start is cleaned, turning through the four directions before moving on.
func cleanRoom(grid [][]int, row, col int) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	cleaned := map[[2]int]bool{}
	dirs := [4][2]int{{-1, 0}, {0, 1}, {1, 0}, {0, -1}}
	var visit func(r, c, dir int)
	visit = func(r, c, dir int) {
		cleaned[[2]int{r, c}] = true
		for turn := 0; turn < 4; turn++ {
			nextDir := (dir + turn) % 4
			nr, nc := r+dirs[nextDir][0], c+dirs[nextDir][1]
			if nr < 0 || nc < 0 || nr >= rows || nc >= cols || grid[nr][nc] != 1 || cleaned[[2]int{nr, nc}] {
				continue
			}
			visit(nr, nc, nextDir)
		}
	}
	visit(row, col, 0)
	return len(cleaned)
}
