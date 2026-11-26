package leetcode

import "strconv"

// LeetCode #694 - Number of Distinct Islands
//
// The first cell of an island, in scan order, anchors its shape, so a translation keeps the same relative cells.
func numDistinctIslands(grid [][]int) int {
	if len(grid) == 0 || len(grid[0]) == 0 {
		return 0
	}
	rows, cols := len(grid), len(grid[0])
	seen := make([][]bool, rows)
	for r := range seen {
		seen[r] = make([]bool, cols)
	}
	shapes := map[string]struct{}{}
	var walk func(r, c, originR, originC int, shape *[]byte)
	walk = func(r, c, originR, originC int, shape *[]byte) {
		if r < 0 || c < 0 || r >= rows || c >= cols || seen[r][c] || grid[r][c] == 0 {
			return
		}
		seen[r][c] = true
		*shape = append(*shape, strconv.Itoa(r-originR)...)
		*shape = append(*shape, ',')
		*shape = append(*shape, strconv.Itoa(c-originC)...)
		*shape = append(*shape, ';')
		walk(r+1, c, originR, originC, shape)
		walk(r-1, c, originR, originC, shape)
		walk(r, c+1, originR, originC, shape)
		walk(r, c-1, originR, originC, shape)
	}
	for r := 0; r < rows; r++ {
		for c := 0; c < cols; c++ {
			if grid[r][c] == 0 || seen[r][c] {
				continue
			}
			shape := []byte{}
			walk(r, c, r, c, &shape)
			shapes[string(shape)] = struct{}{}
		}
	}
	return len(shapes)
}
