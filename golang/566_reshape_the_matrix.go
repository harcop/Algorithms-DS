package leetcode

// LeetCode #566 - Reshape the Matrix
//
// Walk the cells in row-major order and lay them into the new shape when the counts match.
func matrixReshape(mat [][]int, r int, c int) [][]int {
	rows, cols := len(mat), len(mat[0])
	if rows*cols != r*c {
		return mat
	}
	out := make([][]int, r)
	for i := range out {
		out[i] = make([]int, c)
	}
	for i := 0; i < rows*cols; i++ {
		out[i/c][i%c] = mat[i/cols][i%cols]
	}
	return out
}
