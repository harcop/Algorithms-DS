package leetcode

// LeetCode #57 - Insert Interval
//
// Copy intervals that end before the new one, merge every overlap, then append the rest.
func insert(intervals [][]int, newInterval []int) [][]int {
	start, end := newInterval[0], newInterval[1]
	var out [][]int
	i := 0

	for i < len(intervals) && intervals[i][1] < start {
		out = append(out, intervals[i])
		i++
	}
	for i < len(intervals) && intervals[i][0] <= end {
		if intervals[i][0] < start {
			start = intervals[i][0]
		}
		if intervals[i][1] > end {
			end = intervals[i][1]
		}
		i++
	}
	out = append(out, []int{start, end})
	for i < len(intervals) {
		out = append(out, intervals[i])
		i++
	}
	return out
}
