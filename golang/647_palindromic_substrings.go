package leetcode

// LeetCode #647 - Palindromic Substrings
//
// Every index is the center of an odd palindrome and, with the next index, of an even one.
func countSubstrings(s string) int {
	count := 0
	expand := func(left, right int) {
		for left >= 0 && right < len(s) && s[left] == s[right] {
			count++
			left--
			right++
		}
	}
	for i := 0; i < len(s); i++ {
		expand(i, i)
		expand(i, i+1)
	}
	return count
}
