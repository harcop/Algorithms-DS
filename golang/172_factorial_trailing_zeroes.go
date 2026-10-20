package leetcode

// LeetCode #172 - Factorial Trailing Zeroes
//
// Each trailing zero comes from a factor of five in n!.
func trailingZeroes(n int) int {
	ans := 0
	for p := 5; p <= n; p *= 5 {
		ans += n / p
	}
	return ans
}
