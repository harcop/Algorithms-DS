package leetcode

import "math"

// LeetCode #64 - Minimum Path Sum
//
// Each cell takes the cheaper of the path from above and the path from the left.
func minPathSum(grid [][]int) int {
	n := len(grid[0])
	dp := make([]int, n)
	for i := range dp {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0

	for _, row := range grid {
		dp[0] += row[0]
		for col := 1; col < n; col++ {
			if dp[col-1] < dp[col] {
				dp[col] = dp[col-1]
			}
			dp[col] += row[col]
		}
	}
	return dp[n-1]
}
