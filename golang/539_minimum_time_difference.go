package leetcode

import "sort"

// LeetCode #539 - Minimum Time Difference
//
// Sort the minutes of the day, then the answer is the smallest gap, including the wrap from the last time back to the first.
func findMinDifference(timePoints []string) int {
	minutes := make([]int, len(timePoints))
	for i, point := range timePoints {
		hour := int(point[0]-'0')*10 + int(point[1]-'0')
		minute := int(point[3]-'0')*10 + int(point[4]-'0')
		minutes[i] = hour*60 + minute
	}
	sort.Ints(minutes)
	best := 24*60 - minutes[len(minutes)-1] + minutes[0]
	for i := 1; i < len(minutes); i++ {
		best = min(best, minutes[i]-minutes[i-1])
	}
	return best
}
