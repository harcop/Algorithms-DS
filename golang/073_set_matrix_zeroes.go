package leetcode

// LeetCode #73 - Set Matrix Zeroes
//
// Use the first row and column as markers, then zero marked lines after the interior is recorded.
func setZeroes(matrix [][]int) {
	m, n := len(matrix), len(matrix[0])
	row0, col0 := false, false

	for j := 0; j < n; j++ {
		if matrix[0][j] == 0 {
			row0 = true
			break
		}
	}
	for i := 0; i < m; i++ {
		if matrix[i][0] == 0 {
			col0 = true
			break
		}
	}

	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			if matrix[i][j] == 0 {
				matrix[i][0] = 0
				matrix[0][j] = 0
			}
		}
	}

	for i := 1; i < m; i++ {
		if matrix[i][0] == 0 {
			for j := 1; j < n; j++ {
				matrix[i][j] = 0
			}
		}
	}
	for j := 1; j < n; j++ {
		if matrix[0][j] == 0 {
			for i := 1; i < m; i++ {
				matrix[i][j] = 0
			}
		}
	}

	if row0 {
		for j := 0; j < n; j++ {
			matrix[0][j] = 0
		}
	}
	if col0 {
		for i := 0; i < m; i++ {
			matrix[i][0] = 0
		}
	}
}
