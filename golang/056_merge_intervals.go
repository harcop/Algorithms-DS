package leetcode

import "sort"

// LeetCode #56 - Merge Intervals
//
// Sort by start time, then extend the last interval whenever the next one overlaps it.
func merge(intervals [][]int) [][]int {
	if len(intervals) == 0 {
		return nil
	}

	sort.Slice(intervals, func(i, j int) bool {
		return intervals[i][0] < intervals[j][0]
	})

	out := [][]int{intervals[0]}
	for _, cur := range intervals[1:] {
		last := out[len(out)-1]
		if cur[0] <= last[1] {
			if cur[1] > last[1] {
				last[1] = cur[1]
			}
		} else {
			out = append(out, cur)
		}
	}
	return out
}
