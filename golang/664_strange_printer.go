package leetcode

// LeetCode #664 - Strange Printer
//
// A later copy of the same character can be printed in the turn that already printed an earlier one.
func strangePrinter(s string) int {
	n := len(s)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
		dp[i][i] = 1
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length <= n; i++ {
			j := i + length - 1
			dp[i][j] = dp[i][j-1] + 1
			for k := i; k < j; k++ {
				if s[k] != s[j] {
					continue
				}
				mid := 0
				if k+1 <= j-1 {
					mid = dp[k+1][j-1]
				}
				dp[i][j] = min(dp[i][j], dp[i][k]+mid)
			}
		}
	}
	return dp[0][n-1]
}
