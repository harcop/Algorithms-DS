package leetcode

import "strconv"

// LeetCode #471 - Encode String with Shortest Length
//
// LeetCode names this encode; that name already belongs to the string codec (#271).
// A range is either a repeated shorter pattern or the join of two shorter ranges.
func encodeShortest(s string) string {
	n := len(s)
	if n == 0 {
		return s
	}
	dp := make([][]string, n)
	for i := range dp {
		dp[i] = make([]string, n)
		dp[i][i] = s[i : i+1]
	}
	for length := 2; length <= n; length++ {
		for i := 0; i+length <= n; i++ {
			j := i + length - 1
			best := s[i : j+1]
			for k := 1; k <= length/2; k++ {
				if length%k != 0 || !encodeRepeats(s[i:j+1], k) {
					continue
				}
				encoded := strconv.Itoa(length/k) + "[" + dp[i][i+k-1] + "]"
				if len(encoded) < len(best) {
					best = encoded
				}
			}
			for k := i; k < j; k++ {
				if len(dp[i][k])+len(dp[k+1][j]) < len(best) {
					best = dp[i][k] + dp[k+1][j]
				}
			}
			dp[i][j] = best
		}
	}
	return dp[0][n-1]
}

func encodeRepeats(s string, k int) bool {
	unit := s[:k]
	for i := k; i < len(s); i += k {
		if s[i:i+k] != unit {
			return false
		}
	}
	return true
}
