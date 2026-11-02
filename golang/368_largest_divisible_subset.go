package leetcode

import "sort"

// LeetCode #368 - Largest Divisible Subset
//
// After sorting, each value extends the longest earlier subset that divides it.
func largestDivisibleSubset(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}
	sort.Ints(nums)
	size := make([]int, len(nums))
	prev := make([]int, len(nums))
	for i := range nums {
		size[i] = 1
		prev[i] = -1
	}
	best := 0
	for i := range nums {
		for j := 0; j < i; j++ {
			if nums[i]%nums[j] == 0 && size[j]+1 > size[i] {
				size[i] = size[j] + 1
				prev[i] = j
			}
		}
		if size[i] > size[best] {
			best = i
		}
	}
	var out []int
	for best >= 0 {
		out = append(out, nums[best])
		best = prev[best]
	}
	for i, j := 0, len(out)-1; i < j; i, j = i+1, j-1 {
		out[i], out[j] = out[j], out[i]
	}
	return out
}
