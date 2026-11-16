package leetcode

// LeetCode #542 - 01 Matrix
//
// Start from every zero and spread the distance one step at a time.
func updateMatrix(mat [][]int) [][]int {
	if len(mat) == 0 || len(mat[0]) == 0 {
		return nil
	}
	rows, cols := len(mat), len(mat[0])
	dist := make([][]int, rows)
	queue := make([][2]int, 0)
	for r := range mat {
		dist[r] = make([]int, cols)
		for c := range mat[r] {
			if mat[r][c] == 0 {
				queue = append(queue, [2]int{r, c})
				continue
			}
			dist[r][c] = rows + cols
		}
	}
	dirs := [][2]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for head := 0; head < len(queue); head++ {
		cell := queue[head]
		for _, dir := range dirs {
			row, col := cell[0]+dir[0], cell[1]+dir[1]
			if row < 0 || col < 0 || row >= rows || col >= cols || dist[row][col] <= dist[cell[0]][cell[1]]+1 {
				continue
			}
			dist[row][col] = dist[cell[0]][cell[1]] + 1
			queue = append(queue, [2]int{row, col})
		}
	}
	return dist
}
