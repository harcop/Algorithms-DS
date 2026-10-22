package leetcode

import "sort"

// LeetCode #215 - Kth Largest Element in an Array
//
// After sorting, the kth largest sits k places in from the end.
func findKthLargest(nums []int, k int) int {
	sort.Ints(nums)
	return nums[len(nums)-k]
}
