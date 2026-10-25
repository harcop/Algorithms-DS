package leetcode

import "sort"

// LeetCode #259 - 3Sum Smaller
//
// Sort, then count every pair whose sum with the fixed value stays below the target.
func threeSumSmaller(nums []int, target int) int {
	sort.Ints(nums)
	count := 0
	for i := 0; i < len(nums); i++ {
		lo, hi := i+1, len(nums)-1
		for lo < hi {
			if nums[i]+nums[lo]+nums[hi] < target {
				count += hi - lo
				lo++
			} else {
				hi--
			}
		}
	}
	return count
}
