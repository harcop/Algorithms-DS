package leetcode

import "math"

// LeetCode #286 - Walls and Gates
//
// Spread distance outward from every gate, filling empty rooms in order.
func wallsAndGates(rooms [][]int) {
	m := len(rooms)
	if m == 0 {
		return
	}
	n := len(rooms[0])
	queue := make([][2]int, 0)
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			if rooms[i][j] == 0 {
				queue = append(queue, [2]int{i, j})
			}
		}
	}
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for head := 0; head < len(queue); head++ {
		cell := queue[head]
		for _, dir := range dirs {
			row, col := cell[0]+dir[0], cell[1]+dir[1]
			if row < 0 || col < 0 || row >= m || col >= n || rooms[row][col] != math.MaxInt32 {
				continue
			}
			rooms[row][col] = rooms[cell[0]][cell[1]] + 1
			queue = append(queue, [2]int{row, col})
		}
	}
}
