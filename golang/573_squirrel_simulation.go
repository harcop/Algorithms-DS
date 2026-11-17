package leetcode

// LeetCode #573 - Squirrel Simulation
//
// LeetCode names this minDistance; that name already belongs to edit distance (#72).
// Every nut travels to the tree twice, except the first nut, which starts from the squirrel.
func minSquirrelDistance(_, _ int, tree []int, squirrel []int, nuts [][]int) int {
	total, bestSave := 0, -1<<30
	for _, nut := range nuts {
		toTree := manhattan(nut, tree)
		total += toTree * 2
		bestSave = max(bestSave, toTree-manhattan(nut, squirrel))
	}
	return total - bestSave
}

func manhattan(a, b []int) int {
	row, col := a[0]-b[0], a[1]-b[1]
	if row < 0 {
		row = -row
	}
	if col < 0 {
		col = -col
	}
	return row + col
}
