package leetcode

// LeetCode #28 - Find the Index of the First Occurrence in a String
//
// Return the first index where needle occurs in haystack, or -1 when it does not.
func strStr(haystack string, needle string) int {
	if len(needle) == 0 {
		return 0
	}
	if len(needle) > len(haystack) {
		return -1
	}

	for i := 0; i <= len(haystack)-len(needle); i++ {
		if haystack[i:i+len(needle)] == needle {
			return i
		}
	}
	return -1
}
