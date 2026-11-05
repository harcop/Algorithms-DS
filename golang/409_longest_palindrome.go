package leetcode

// LeetCode #409 - Longest Palindrome
//
// LeetCode names this longestPalindrome; that name already belongs to the substring search (#5).
// Pair every even character count, then add one center if any character has a leftover.
func longestPalindromeLength(s string) int {
	count := [128]int{}
	for i := 0; i < len(s); i++ {
		count[s[i]]++
	}
	length := 0
	odd := false
	for _, c := range count {
		length += c / 2 * 2
		if c%2 == 1 {
			odd = true
		}
	}
	if odd {
		return length + 1
	}
	return length
}
