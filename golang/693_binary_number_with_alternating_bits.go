package leetcode

// LeetCode #693 - Binary Number with Alternating Bits
//
// Shifting the number once and xoring it turns alternating bits into a solid run of ones.
func hasAlternatingBits(n int) bool {
	xored := n ^ (n >> 1)
	return xored&(xored+1) == 0
}
