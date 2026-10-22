package leetcode

// LeetCode #214 - Shortest Palindrome
//
// The longest palindromic prefix tells how many characters must be added in front.
func shortestPalindrome(s string) string {
	if s == "" {
		return s
	}
	n := len(s)
	rev := make([]byte, n)
	for i := 0; i < n; i++ {
		rev[i] = s[n-1-i]
	}
	combined := s + "#" + string(rev)
	lps := make([]int, len(combined))
	length := 0
	for i := 1; i < len(combined); i++ {
		for length > 0 && combined[i] != combined[length] {
			length = lps[length-1]
		}
		if combined[i] == combined[length] {
			length++
		}
		lps[i] = length
	}
	return string(rev[:n-lps[len(lps)-1]]) + s
}
