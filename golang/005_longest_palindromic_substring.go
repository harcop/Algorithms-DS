package leetcode

// LeetCode #5 - Longest Palindromic Substring
//
// Given a string s, return the longest palindromic substring in s.
// Constraints mention English letters and digits (ASCII).
func longestPalindrome(s string) string {
	n := len(s)
	if n == 0 {
		return ""
	}

	start := 0
	maxLen := 1

	expand := func(left, right int) (int, int) {
		for left >= 0 && right < n && s[left] == s[right] {
			left--
			right++
		}
		return left + 1, right
	}

	for i := 0; i < n; i++ {
		l1, r1 := expand(i, i)
		if r1-l1 > maxLen {
			maxLen = r1 - l1
			start = l1
		}
		if i+1 < n {
			l2, r2 := expand(i, i+1)
			if r2-l2 > maxLen {
				maxLen = r2 - l2
				start = l2
			}
		}
	}

	return s[start : start+maxLen]
}
