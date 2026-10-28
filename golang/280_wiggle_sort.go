package leetcode

import "sort"

// LeetCode #280 - Wiggle Sort
//
// Sort, then swap each adjacent pair so the sequence rises and falls.
func wiggleSort(nums []int) {
	sort.Ints(nums)
	for i := 1; i+1 < len(nums); i += 2 {
		nums[i], nums[i+1] = nums[i+1], nums[i]
	}
}
