package leetcode

// LeetCode #392 - Is Subsequence
//
// Walk the longer string and take the next needed character whenever it appears.
func isSubsequence(s, t string) bool {
	i := 0
	for j := 0; j < len(t) && i < len(s); j++ {
		if s[i] == t[j] {
			i++
		}
	}
	return i == len(s)
}
