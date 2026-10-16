package leetcode

// LeetCode #96 - Unique Binary Search Trees
//
// The count for n is the sum, over each root, of left-count times right-count.
func numTrees(n int) int {
	dp := make([]int, n+1)
	dp[0] = 1
	if n == 0 {
		return 1
	}
	dp[1] = 1
	for nodes := 2; nodes <= n; nodes++ {
		for root := 1; root <= nodes; root++ {
			dp[nodes] += dp[root-1] * dp[nodes-root]
		}
	}
	return dp[n]
}
