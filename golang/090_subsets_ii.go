package leetcode

import "sort"

// LeetCode #90 - Subsets II
//
// Build subsets in sorted order, and only extend the subsets created by the previous duplicate.
func subsetsWithDup(nums []int) [][]int {
	sort.Ints(nums)
	res := [][]int{{}}
	start := 0

	for i := 0; i < len(nums); i++ {
		startIdx := 0
		if i > 0 && nums[i] == nums[i-1] {
			startIdx = start
		}
		start = len(res)
		added := make([][]int, 0, len(res)-startIdx)
		for j := startIdx; j < len(res); j++ {
			s := append(append([]int(nil), res[j]...), nums[i])
			added = append(added, s)
		}
		res = append(res, added...)
	}
	return res
}
