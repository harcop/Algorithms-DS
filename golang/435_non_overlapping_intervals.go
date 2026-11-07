package leetcode

import "sort"

// LeetCode #435 - Non-overlapping Intervals
//
// Keeping the interval that ends first leaves the most room for the ones that follow.
func eraseOverlapIntervals(intervals [][]int) int {
	if len(intervals) == 0 {
		return 0
	}
	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][1] < intervals[j][1]
	})
	end := intervals[0][1]
	remove := 0
	for _, interval := range intervals[1:] {
		if interval[0] < end {
			remove++
			continue
		}
		end = interval[1]
	}
	return remove
}
