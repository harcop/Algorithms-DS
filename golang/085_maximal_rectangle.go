package leetcode

// LeetCode #85 - Maximal Rectangle
//
// Treat each row as the base of a histogram of consecutive ones, then take the largest rectangle.
func maximalRectangle(matrix [][]byte) int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return 0
	}
	heights := make([]int, len(matrix[0]))
	best := 0
	for _, row := range matrix {
		for j, cell := range row {
			if cell == '1' {
				heights[j]++
			} else {
				heights[j] = 0
			}
		}
		if area := largestRectangleArea(heights); area > best {
			best = area
		}
	}
	return best
}
