package leetcode

// LeetCode #201 - Bitwise AND of Numbers Range
//
// The AND of a range is the common prefix of the two ends, with the rest zero.
func rangeBitwiseAnd(left int, right int) int {
	l, r := uint32(left), uint32(right)
	shift := uint32(0)
	for l != r {
		l >>= 1
		r >>= 1
		shift++
	}
	return int(l << shift)
}
