package leetcode

// LeetCode #240 - Search a 2D Matrix II
//
// LeetCode names this searchMatrix; that name already belongs to the fully sorted matrix (#74).
// Start at the top right and discard a row or a column at each step.
func searchMatrixII(matrix [][]int, target int) bool {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return false
	}
	row, col := 0, len(matrix[0])-1
	for row < len(matrix) && col >= 0 {
		value := matrix[row][col]
		if value == target {
			return true
		}
		if value > target {
			col--
		} else {
			row++
		}
	}
	return false
}
