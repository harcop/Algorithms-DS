package leetcode

import "strconv"

// LeetCode #163 - Missing Ranges
//
// Walk the sorted numbers and report every gap between lower and upper.
func findMissingRanges(nums []int, lower int, upper int) []string {
	var out []string
	prev := int64(lower) - 1
	add := func(lo, hi int64) {
		if lo > hi {
			return
		}
		if lo == hi {
			out = append(out, strconv.FormatInt(lo, 10))
		} else {
			out = append(out, strconv.FormatInt(lo, 10)+"->"+strconv.FormatInt(hi, 10))
		}
	}
	for _, value := range nums {
		current := int64(value)
		if current > prev+1 {
			add(prev+1, current-1)
		}
		prev = current
	}
	if int64(upper) > prev {
		add(prev+1, int64(upper))
	}
	return out
}
