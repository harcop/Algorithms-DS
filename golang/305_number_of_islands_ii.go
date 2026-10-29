package leetcode

// LeetCode #305 - Number of Islands II
//
// Add each land cell and merge it with neighboring land, then record the island count.
func numIslands2(m int, n int, positions [][]int) []int {
	parent := make([]int, m*n)
	for i := range parent {
		parent[i] = i
	}
	land := make([]bool, m*n)
	count := 0
	var find func(int) int
	find = func(x int) int {
		if parent[x] != x {
			parent[x] = find(parent[x])
		}
		return parent[x]
	}
	union := func(a, b int) bool {
		a, b = find(a), find(b)
		if a == b {
			return false
		}
		parent[b] = a
		return true
	}
	dirs := [][2]int{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}
	out := make([]int, 0, len(positions))
	for _, pos := range positions {
		row, col := pos[0], pos[1]
		id := row*n + col
		if land[id] {
			out = append(out, count)
			continue
		}
		land[id] = true
		count++
		for _, dir := range dirs {
			nextRow, nextCol := row+dir[0], col+dir[1]
			if nextRow < 0 || nextCol < 0 || nextRow >= m || nextCol >= n {
				continue
			}
			nextID := nextRow*n + nextCol
			if land[nextID] && union(id, nextID) {
				count--
			}
		}
		out = append(out, count)
	}
	return out
}
