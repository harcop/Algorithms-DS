package leetcode

import "sort"

// LeetCode #442 - Find All Duplicates in an Array
//
// Mark each value's index as seen by negating it, and record a value the second time it appears.
func findDuplicates(nums []int) []int {
	var out []int
	for _, value := range nums {
		index := absInt(value) - 1
		if nums[index] < 0 {
			out = append(out, index+1)
			continue
		}
		nums[index] = -nums[index]
	}
	sort.Ints(out)
	return out
}
