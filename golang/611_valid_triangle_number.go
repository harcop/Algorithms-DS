package leetcode

import "sort"

// LeetCode #611 - Valid Triangle Number
//
// With sides sorted, every left partner of a valid right end also works, so the window slides inward.
func triangleNumber(nums []int) int {
	sort.Ints(nums)
	count := 0
	for k := len(nums) - 1; k >= 2; k-- {
		i, j := 0, k-1
		for i < j {
			if nums[i]+nums[j] > nums[k] {
				count += j - i
				j--
			} else {
				i++
			}
		}
	}
	return count
}
