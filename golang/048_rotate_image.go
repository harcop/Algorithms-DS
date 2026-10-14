package leetcode

// LeetCode #48 - Rotate Image
//
// Transpose across the main diagonal, then reverse each row to rotate 90 degrees clockwise.
func rotate(matrix [][]int) {
	n := len(matrix)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]
		}
	}
	for _, row := range matrix {
		for left, right := 0, n-1; left < right; left, right = left+1, right-1 {
			row[left], row[right] = row[right], row[left]
		}
	}
}
