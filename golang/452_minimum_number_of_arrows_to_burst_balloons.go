package leetcode

import "sort"

// LeetCode #452 - Minimum Number of Arrows to Burst Balloons
//
// Sort balloons by their end point and shoot the next arrow only when a balloon starts after the current shot.
func findMinArrowShots(points [][]int) int {
	if len(points) == 0 {
		return 0
	}
	sort.Slice(points, func(i, j int) bool {
		return points[i][1] < points[j][1]
	})
	arrows := 1
	end := points[0][1]
	for _, point := range points[1:] {
		if point[0] > end {
			arrows++
			end = point[1]
			continue
		}
		if point[1] < end {
			end = point[1]
		}
	}
	return arrows
}
