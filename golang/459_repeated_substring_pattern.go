package leetcode

import "strings"

// LeetCode #459 - Repeated Substring Pattern
//
// The string repeats when it occurs again inside the doubled string with both ends removed.
func repeatedSubstringPattern(s string) bool {
	if len(s) < 2 {
		return false
	}
	doubled := s + s
	return strings.Contains(doubled[1:len(doubled)-1], s)
}
