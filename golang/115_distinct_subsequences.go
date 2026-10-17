package leetcode

// LeetCode #115 - Distinct Subsequences
//
// Each character of s can extend every way of matching the prefix of t that ends with it.
func numDistinct(s string, t string) int {
	if len(t) == 0 {
		return 1
	}
	if len(s) < len(t) {
		return 0
	}

	dp := make([]int, len(t)+1)
	dp[0] = 1
	for i := 0; i < len(s); i++ {
		for j := len(t); j >= 1; j-- {
			if s[i] == t[j-1] {
				dp[j] += dp[j-1]
			}
		}
	}
	return dp[len(t)]
}
