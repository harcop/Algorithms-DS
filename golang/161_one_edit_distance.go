package leetcode

// LeetCode #161 - One Edit Distance
//
// Walk both strings once and allow a single insert, delete, or replace.
func isOneEditDistance(s string, t string) bool {
	if len(s) < len(t) {
		return isOneEditDistance(t, s)
	}
	if len(s)-len(t) > 1 {
		return false
	}
	i, j, edits := 0, 0, 0
	for i < len(s) && j < len(t) {
		if s[i] == t[j] {
			i++
			j++
			continue
		}
		edits++
		if edits > 1 {
			return false
		}
		if len(s) > len(t) {
			i++
		} else {
			i++
			j++
		}
	}
	edits += len(s) - i + len(t) - j
	return edits == 1
}
