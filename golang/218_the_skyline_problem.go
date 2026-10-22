package leetcode

import "sort"

// LeetCode #218 - The Skyline Problem
//
// Sweep left edges and right edges, and record a point whenever the tallest live building changes.
func getSkyline(buildings [][]int) [][]int {
	events := make([][2]int, 0, len(buildings)*2)
	for _, building := range buildings {
		events = append(events, [2]int{building[0], -building[2]}, [2]int{building[1], building[2]})
	}
	sort.Slice(events, func(i, j int) bool {
		if events[i][0] != events[j][0] {
			return events[i][0] < events[j][0]
		}
		return events[i][1] < events[j][1]
	})
	count := map[int]int{0: 1}
	prev := 0
	var out [][]int
	for i := 0; i < len(events); {
		x := events[i][0]
		for i < len(events) && events[i][0] == x {
			height := events[i][1]
			if height < 0 {
				count[-height]++
			} else {
				count[height]--
				if count[height] == 0 {
					delete(count, height)
				}
			}
			i++
		}
		cur := 0
		for height := range count {
			if height > cur {
				cur = height
			}
		}
		if cur != prev {
			out = append(out, []int{x, cur})
			prev = cur
		}
	}
	return out
}
