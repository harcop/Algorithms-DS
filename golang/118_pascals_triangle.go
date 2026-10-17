package leetcode

// LeetCode #118 - Pascal's Triangle
//
// Each inner entry is the sum of the two entries above it.
func generate(numRows int) [][]int {
	out := make([][]int, 0, numRows)
	for i := 0; i < numRows; i++ {
		row := make([]int, i+1)
		row[0], row[i] = 1, 1
		for j := 1; j < i; j++ {
			row[j] = out[i-1][j-1] + out[i-1][j]
		}
		out = append(out, row)
	}
	return out
}
