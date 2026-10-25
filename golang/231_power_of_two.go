package leetcode

// LeetCode #231 - Power of Two
//
// A positive power of two has exactly one bit set.
func isPowerOfTwo(n int) bool {
	return n > 0 && n&(n-1) == 0
}
