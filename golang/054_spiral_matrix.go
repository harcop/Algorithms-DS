package leetcode

// LeetCode #54 - Spiral Matrix
//
// Walk the outer ring right, down, left, then up, and shrink the bounds after each side.
func spiralOrder(matrix [][]int) []int {
	if len(matrix) == 0 || len(matrix[0]) == 0 {
		return nil
	}

	top, bottom := 0, len(matrix)-1
	left, right := 0, len(matrix[0])-1
	out := make([]int, 0, len(matrix)*len(matrix[0]))

	for left <= right && top <= bottom {
		for c := left; c <= right; c++ {
			out = append(out, matrix[top][c])
		}
		top++

		for r := top; r <= bottom; r++ {
			out = append(out, matrix[r][right])
		}
		right--

		if top <= bottom {
			for c := right; c >= left; c-- {
				out = append(out, matrix[bottom][c])
			}
			bottom--
		}

		if left <= right {
			for r := bottom; r >= top; r-- {
				out = append(out, matrix[r][left])
			}
			left++
		}
	}

	return out
}
