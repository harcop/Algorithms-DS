package leetcode

import "sort"

// LeetCode #698 - Partition to K Equal Sum Subsets
//
// The unused number with the smallest index must open the next subset, and equal sums fill k of them.
func canPartitionKSubsets(nums []int, k int) bool {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	if k == 0 || sum%k != 0 {
		return false
	}
	target := sum / k
	sort.Slice(nums, func(i, j int) bool { return nums[i] > nums[j] })
	if nums[0] > target {
		return false
	}
	used := make([]bool, len(nums))
	var search func(start, filled, remain int) bool
	search = func(start, filled, remain int) bool {
		if filled == k {
			return true
		}
		if remain == 0 {
			return search(0, filled+1, target)
		}
		for i := start; i < len(nums); i++ {
			if used[i] || nums[i] > remain {
				continue
			}
			used[i] = true
			if search(i+1, filled, remain-nums[i]) {
				return true
			}
			used[i] = false
			if remain == target {
				return false
			}
			for i+1 < len(nums) && nums[i+1] == nums[i] {
				i++
			}
		}
		return false
	}
	return search(0, 0, target)
}
