package leetcode

// LeetCode #311 - Sparse Matrix Multiplication
//
// LeetCode names this multiply; that name already belongs to string multiplication (#43).
// Skip zeros in the first matrix and add the matching products into the result.
func multiplySparse(mat1 [][]int, mat2 [][]int) [][]int {
	m, k, n := len(mat1), len(mat1[0]), len(mat2[0])
	out := make([][]int, m)
	for i := range out {
		out[i] = make([]int, n)
	}
	for i := 0; i < m; i++ {
		for p := 0; p < k; p++ {
			if mat1[i][p] == 0 {
				continue
			}
			for j := 0; j < n; j++ {
				out[i][j] += mat1[i][p] * mat2[p][j]
			}
		}
	}
	return out
}
