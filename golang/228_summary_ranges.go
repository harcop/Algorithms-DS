package leetcode

import "strconv"

// LeetCode #228 - Summary Ranges
//
// Collapse each run of consecutive numbers into a single range string.
func summaryRanges(nums []int) []string {
	var out []string
	for i := 0; i < len(nums); {
		start := i
		for i+1 < len(nums) && int64(nums[i+1]) == int64(nums[i])+1 {
			i++
		}
		if start == i {
			out = append(out, strconv.Itoa(nums[start]))
		} else {
			out = append(out, strconv.Itoa(nums[start])+"->"+strconv.Itoa(nums[i]))
		}
		i++
	}
	return out
}
