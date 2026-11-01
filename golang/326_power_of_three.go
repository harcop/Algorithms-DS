package leetcode

// LeetCode #326 - Power of Three
//
// 3^19 is the largest power of three that fits in a 32-bit integer, so it divides only powers of three.
func isPowerOfThree(n int) bool {
	return n > 0 && 1162261467%n == 0
}
