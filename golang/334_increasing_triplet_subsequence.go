package leetcode

import "math"

// LeetCode #334 - Increasing Triplet Subsequence
//
// Keep the smallest start and the smallest middle, and a later larger value completes the triplet.
func increasingTriplet(nums []int) bool {
	small, mid := math.MaxInt32, math.MaxInt32
	for _, value := range nums {
		if value <= small {
			small = value
		} else if value <= mid {
			mid = value
		} else {
			return true
		}
	}
	return false
}
