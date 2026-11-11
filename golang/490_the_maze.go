package leetcode

// LeetCode #490 - The Maze
//
// The ball rolls in a straight line until a wall, and a search records each cell where it can stop.
func hasPath(maze [][]int, start []int, destination []int) bool {
	rows, cols := len(maze), len(maze[0])
	seen := make([][]bool, rows)
	for r := range seen {
		seen[r] = make([]bool, cols)
	}
	queue := [][2]int{{start[0], start[1]}}
	seen[start[0]][start[1]] = true
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for len(queue) > 0 {
		cell := queue[0]
		queue = queue[1:]
		if cell[0] == destination[0] && cell[1] == destination[1] {
			return true
		}
		for _, dir := range dirs {
			row, col := cell[0], cell[1]
			for {
				nextRow, nextCol := row+dir[0], col+dir[1]
				if nextRow < 0 || nextCol < 0 || nextRow >= rows || nextCol >= cols || maze[nextRow][nextCol] != 0 {
					break
				}
				row, col = nextRow, nextCol
			}
			if !seen[row][col] {
				seen[row][col] = true
				queue = append(queue, [2]int{row, col})
			}
		}
	}
	return false
}
