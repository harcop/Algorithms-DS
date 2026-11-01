package leetcode

// LeetCode #343 - Integer Break
//
// Breaking into as many 3s as possible, avoiding a leftover 1, maximizes the product.
func integerBreak(n int) int {
	if n <= 3 {
		return n - 1
	}
	prod := 1
	for n > 4 {
		prod *= 3
		n -= 3
	}
	return prod * n
}
