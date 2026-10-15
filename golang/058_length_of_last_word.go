package leetcode

import "strings"

// LeetCode #58 - Length of Last Word
//
// Split on whitespace and return the length of the final word.
func lengthOfLastWord(s string) int {
	fields := strings.Fields(s)
	if len(fields) == 0 {
		return 0
	}
	return len(fields[len(fields)-1])
}
