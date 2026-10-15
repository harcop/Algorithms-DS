package leetcode

// LeetCode #59 - Spiral Matrix II
//
// Write increasing values around each ring, then move the bounds inward.
func generateMatrix(n int) [][]int {
	matrix := make([][]int, n)
	for i := range matrix {
		matrix[i] = make([]int, n)
	}

	top, bottom := 0, n-1
	left, right := 0, n-1
	val := 1

	for left <= right && top <= bottom {
		for c := left; c <= right; c++ {
			matrix[top][c] = val
			val++
		}
		top++
		for r := top; r <= bottom; r++ {
			matrix[r][right] = val
			val++
		}
		right--
		if top <= bottom {
			for c := right; c >= left; c-- {
				matrix[bottom][c] = val
				val++
			}
			bottom--
		}
		if left <= right {
			for r := bottom; r >= top; r-- {
				matrix[r][left] = val
				val++
			}
			left++
		}
	}

	return matrix
}
