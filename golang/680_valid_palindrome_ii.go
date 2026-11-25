package leetcode

// LeetCode #680 - Valid Palindrome II
//
// A mismatch can spend the one deletion on either side, and the rest must already be a palindrome.
func validPalindrome(s string) bool {
	return palindromeWithSkip(s, 0, len(s)-1, true)
}

func palindromeWithSkip(s string, left, right int, canSkip bool) bool {
	for left < right {
		if s[left] == s[right] {
			left++
			right--
			continue
		}
		if !canSkip {
			return false
		}
		return palindromeWithSkip(s, left+1, right, false) || palindromeWithSkip(s, left, right-1, false)
	}
	return true
}
