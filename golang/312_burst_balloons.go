package leetcode

// LeetCode #312 - Burst Balloons
//
// The last balloon burst between two walls scores those walls times the balloon.
func maxCoins(nums []int) int {
	balloons := make([]int, 0, len(nums)+2)
	balloons = append(balloons, 1)
	balloons = append(balloons, nums...)
	balloons = append(balloons, 1)
	n := len(balloons)
	dp := make([][]int, n)
	for i := range dp {
		dp[i] = make([]int, n)
	}
	for length := 2; length < n; length++ {
		for left := 0; left+length < n; left++ {
			right := left + length
			for k := left + 1; k < right; k++ {
				coins := dp[left][k] + dp[k][right] + balloons[left]*balloons[k]*balloons[right]
				if coins > dp[left][right] {
					dp[left][right] = coins
				}
			}
		}
	}
	return dp[0][n-1]
}
