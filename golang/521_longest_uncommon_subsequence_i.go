package leetcode

// LeetCode #521 - Longest Uncommon Subsequence I
//
// Two equal strings have no uncommon subsequence, and otherwise the longer string itself is one.
func findLUSlength(a, b string) int {
	if a == b {
		return -1
	}
	return max(len(a), len(b))
}
