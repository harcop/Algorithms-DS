package leetcode

// LeetCode #205 - Isomorphic Strings
//
// Two strings match when each character maps to exactly one character in the other.
func isIsomorphic(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	var toT, toS [256]int
	for i := range toT {
		toT[i] = -1
		toS[i] = -1
	}
	for i := 0; i < len(s); i++ {
		from, to := int(s[i]), int(t[i])
		if toT[from] == -1 && toS[to] == -1 {
			toT[from] = to
			toS[to] = from
			continue
		}
		if toT[from] != to || toS[to] != from {
			return false
		}
	}
	return true
}
