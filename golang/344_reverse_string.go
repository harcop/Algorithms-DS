package leetcode

// LeetCode #344 - Reverse String
//
// Swap characters from both ends until they meet.
func reverseString(s []byte) {
	for i, j := 0, len(s)-1; i < j; i, j = i+1, j-1 {
		s[i], s[j] = s[j], s[i]
	}
}
