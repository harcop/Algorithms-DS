package leetcode

// LeetCode #474 - Ones and Zeroes
//
// Each string spends its zeros and ones, and a backward knapsack keeps the largest set that still fits.
func findMaxForm(strs []string, m int, n int) int {
	dp := make([][]int, m+1)
	for i := range dp {
		dp[i] = make([]int, n+1)
	}
	for _, s := range strs {
		zeros, ones := 0, 0
		for i := 0; i < len(s); i++ {
			if s[i] == '0' {
				zeros++
				continue
			}
			ones++
		}
		if zeros > m || ones > n {
			continue
		}
		for i := m; i >= zeros; i-- {
			for j := n; j >= ones; j-- {
				dp[i][j] = max(dp[i][j], dp[i-zeros][j-ones]+1)
			}
		}
	}
	return dp[m][n]
}
