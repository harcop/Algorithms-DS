package leetcode

// LeetCode #377 - Combination Sum IV
//
// Order matters, so each total counts every number that can finish a smaller total.
func combinationSum4(nums []int, target int) int {
	dp := make([]int, target+1)
	dp[0] = 1
	for sum := 1; sum <= target; sum++ {
		for _, num := range nums {
			if num <= sum {
				dp[sum] += dp[sum-num]
			}
		}
	}
	return dp[target]
}
