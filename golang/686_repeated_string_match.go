package leetcode

import "strings"

// LeetCode #686 - Repeated String Match
//
// Repeating a until it is at least as long as b covers every alignment except one extra copy.
func repeatedStringMatch(a, b string) int {
	if a == "" {
		return -1
	}
	built := a
	count := 1
	for len(built) < len(b) {
		built += a
		count++
	}
	if strings.Contains(built, b) {
		return count
	}
	if strings.Contains(built+a, b) {
		return count + 1
	}
	return -1
}
