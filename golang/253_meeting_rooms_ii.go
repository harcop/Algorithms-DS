package leetcode

import "sort"

// LeetCode #253 - Meeting Rooms II
//
// Sort starts and ends, and reuse a room as soon as an earlier meeting has finished.
func minMeetingRooms(intervals [][]int) int {
	starts := make([]int, len(intervals))
	ends := make([]int, len(intervals))
	for i, interval := range intervals {
		starts[i] = interval[0]
		ends[i] = interval[1]
	}
	sort.Ints(starts)
	sort.Ints(ends)
	rooms, ended, best := 0, 0, 0
	for _, start := range starts {
		for ended < len(ends) && ends[ended] <= start {
			rooms--
			ended++
		}
		rooms++
		if rooms > best {
			best = rooms
		}
	}
	return best
}
