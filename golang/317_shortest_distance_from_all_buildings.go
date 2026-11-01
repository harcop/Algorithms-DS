package leetcode

import "math"

// LeetCode #317 - Shortest Distance from All Buildings
//
// LeetCode names this shortestDistance; that name already belongs to the word-distance scan (#243).
// Sum the distance from every building and keep an empty cell that every building can reach.
func shortestDistanceBuildings(grid [][]int) int {
	rows := len(grid)
	if rows == 0 {
		return -1
	}
	cols := len(grid[0])
	distSum := make([][]int, rows)
	reach := make([][]int, rows)
	for i := range distSum {
		distSum[i] = make([]int, cols)
		reach[i] = make([]int, cols)
	}
	buildings := 0
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			if grid[row][col] != 1 {
				continue
			}
			buildings++
			visited := make([][]bool, rows)
			for i := range visited {
				visited[i] = make([]bool, cols)
			}
			visited[row][col] = true
			type step struct{ row, col, dist int }
			queue := []step{{row, col, 0}}
			for head := 0; head < len(queue); head++ {
				current := queue[head]
				for _, dir := range dirs {
					nextRow, nextCol := current.row+dir[0], current.col+dir[1]
					if nextRow < 0 || nextCol < 0 || nextRow >= rows || nextCol >= cols {
						continue
					}
					if grid[nextRow][nextCol] != 0 || visited[nextRow][nextCol] {
						continue
					}
					visited[nextRow][nextCol] = true
					nextDist := current.dist + 1
					distSum[nextRow][nextCol] += nextDist
					reach[nextRow][nextCol]++
					queue = append(queue, step{nextRow, nextCol, nextDist})
				}
			}
		}
	}
	best := math.MaxInt32
	for row := 0; row < rows; row++ {
		for col := 0; col < cols; col++ {
			if grid[row][col] == 0 && reach[row][col] == buildings && distSum[row][col] < best {
				best = distSum[row][col]
			}
		}
	}
	if best == math.MaxInt32 {
		return -1
	}
	return best
}
