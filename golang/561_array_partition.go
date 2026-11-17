package leetcode

import "sort"

// LeetCode #561 - Array Partition
//
// After sorting, every other value is the smaller one in an optimal pair.
func arrayPairSum(nums []int) int {
	sort.Ints(nums)
	sum := 0
	for i := 0; i < len(nums); i += 2 {
		sum += nums[i]
	}
	return sum
}
