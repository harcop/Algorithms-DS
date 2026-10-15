package leetcode

// LeetCode #63 - Unique Paths II
//
// Same row of path counts as unique paths, with obstacles forced to zero.
func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	n := len(obstacleGrid[0])
	dp := make([]int, n)
	if obstacleGrid[0][0] == 0 {
		dp[0] = 1
	}

	for _, row := range obstacleGrid {
		for col := 0; col < n; col++ {
			if row[col] == 1 {
				dp[col] = 0
			} else if col > 0 {
				dp[col] += dp[col-1]
			}
		}
	}
	return dp[n-1]
}
