package leetcode

// LeetCode #191 - Number of 1 Bits
//
// Clear the lowest set bit until none remain.
func hammingWeight(n uint32) int {
	count := 0
	for n != 0 {
		n &= n - 1
		count++
	}
	return count
}
