package leetcode

// LeetCode #221 - Maximal Square
//
// A square of 1s ending here is one larger than the smallest square touching its top and left.
func maximalSquare(matrix [][]byte) int {
	m := len(matrix)
	if m == 0 {
		return 0
	}
	n := len(matrix[0])
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	best := 0
	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			if matrix[i-1][j-1] != '1' {
				continue
			}
			dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1
			if dp[i][j] > best {
				best = dp[i][j]
			}
		}
	}
	return best * best
}
