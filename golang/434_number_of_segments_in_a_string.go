package leetcode

import "strings"

// LeetCode #434 - Number of Segments in a String
//
// A segment is a run of non-space characters.
func countSegments(s string) int {
	return len(strings.Fields(s))
}
