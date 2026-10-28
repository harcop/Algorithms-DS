package leetcode

// LeetCode #279 - Perfect Squares
//
// The fewest squares for n is one more than the fewest for n minus some square.
func numSquares(n int) int {
	dp := make([]int, n+1)
	for i := 1; i <= n; i++ {
		best := i
		for j := 1; j*j <= i; j++ {
			if dp[i-j*j] < best {
				best = dp[i-j*j]
			}
		}
		dp[i] = best + 1
	}
	return dp[n]
}
