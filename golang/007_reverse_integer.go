package leetcode

import "math"

// LeetCode #7 - Reverse Integer
//
// Given a signed 32-bit integer x, return x with its digits reversed.
// If reversing causes overflow, return 0.
func reverse(x int) int {
	n := x
	rev := 0

	for n != 0 {
		rev = rev*10 + n%10
		n /= 10
	}

	if rev < math.MinInt32 || rev > math.MaxInt32 {
		return 0
	}
	return rev
}
