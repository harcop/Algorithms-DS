package leetcode

// LeetCode #651 - 4 Keys Keyboard
//
// After a buffer is copied, each later paste multiplies the letters that were on the screen.
func maxA(n int) int {
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		dp[i] = i
		for j := 1; j <= i-3; j++ {
			dp[i] = max(dp[i], dp[j]*(i-j-1))
		}
	}
	return dp[n]
}
