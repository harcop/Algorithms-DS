package leetcode

// LeetCode #304 - Range Sum Query 2D - Immutable
//
// A 2D prefix sum answers a rectangle from four corner lookups.
type NumMatrix struct {
	prefix [][]int
}

func NewNumMatrix(matrix [][]int) NumMatrix {
	m := len(matrix)
	n := 0
	if m > 0 {
		n = len(matrix[0])
	}
	prefix := make([][]int, m+1)
	for i := range prefix {
		prefix[i] = make([]int, n+1)
	}
	for i := 0; i < m; i++ {
		for j := 0; j < n; j++ {
			prefix[i+1][j+1] = matrix[i][j] + prefix[i][j+1] + prefix[i+1][j] - prefix[i][j]
		}
	}
	return NumMatrix{prefix: prefix}
}

func (matrix NumMatrix) SumRegion(row1, col1, row2, col2 int) int {
	return matrix.prefix[row2+1][col2+1] - matrix.prefix[row1][col2+1] - matrix.prefix[row2+1][col1] + matrix.prefix[row1][col1]
}
