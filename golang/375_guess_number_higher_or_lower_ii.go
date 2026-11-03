package leetcode

import "math"

// LeetCode #375 - Guess Number Higher or Lower II
//
// For each range, choose the guess that leaves the cheaper worst-case remainder.
func getMoneyAmount(n int) int {
	dp := make([][]int, n+2)
	for i := range dp {
		dp[i] = make([]int, n+2)
	}
	for length := 2; length <= n; length++ {
		for i := 1; i <= n+1-length; i++ {
			j := i + length - 1
			best := math.MaxInt32
			for x := i; x <= j; x++ {
				cost := dp[i][x-1]
				if dp[x+1][j] > cost {
					cost = dp[x+1][j]
				}
				cost += x
				if cost < best {
					best = cost
				}
			}
			dp[i][j] = best
		}
	}
	return dp[1][n]
}
