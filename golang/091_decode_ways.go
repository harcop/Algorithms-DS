package leetcode

// LeetCode #91 - Decode Ways
//
// A digit can stand alone, or pair with the previous digit when the pair is from 10 to 26.
func numDecodings(s string) int {
	n := len(s)
	if n == 0 || s[0] == '0' {
		return 0
	}

	dp := make([]int, n+1)
	dp[0], dp[1] = 1, 1
	for i := 2; i <= n; i++ {
		if s[i-1] != '0' {
			dp[i] += dp[i-1]
		}
		pair := int(s[i-2]-'0')*10 + int(s[i-1]-'0')
		if pair >= 10 && pair <= 26 {
			dp[i] += dp[i-2]
		}
	}
	return dp[n]
}
