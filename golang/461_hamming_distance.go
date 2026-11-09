package leetcode

import "math/bits"

// LeetCode #461 - Hamming Distance
//
// The distance is the number of bits that differ, which is the number of one-bits in the xor.
func hammingDistance(x, y int) int {
	return bits.OnesCount(uint(x) ^ uint(y))
}
