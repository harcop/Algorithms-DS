package leetcode

import "sort"

// LeetCode #436 - Find Right Interval
//
// Sort the start points, then binary-search the first start that meets each interval's end.
func findRightInterval(intervals [][]int) []int {
	n := len(intervals)
	starts := make([][2]int, n)
	for i, interval := range intervals {
		starts[i] = [2]int{interval[0], i}
	}
	sort.Slice(starts, func(i, j int) bool {
		return starts[i][0] < starts[j][0]
	})
	ans := make([]int, n)
	for i, interval := range intervals {
		end := interval[1]
		j := sort.Search(n, func(k int) bool {
			return starts[k][0] >= end
		})
		if j == n {
			ans[i] = -1
		} else {
			ans[i] = starts[j][1]
		}
	}
	return ans
}
