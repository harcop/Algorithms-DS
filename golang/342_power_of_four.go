package leetcode

// LeetCode #342 - Power of Four
//
// A power of four is a power of two whose only set bit sits in an odd position.
func isPowerOfFour(n int) bool {
	return n > 0 && n&(n-1) == 0 && n&0x55555555 != 0
}
