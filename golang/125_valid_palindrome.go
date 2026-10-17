package leetcode

// LeetCode #125 - Valid Palindrome
//
// LeetCode names this isPalindrome; that name already belongs to checking an integer (#9).
// Compare the string from both ends after keeping only lowercase letters and digits.
func isPalindromeString(s string) bool {
	filtered := make([]byte, 0, len(s))
	for i := 0; i < len(s); i++ {
		c := s[i]
		switch {
		case c >= 'A' && c <= 'Z':
			filtered = append(filtered, c-'A'+'a')
		case c >= 'a' && c <= 'z', c >= '0' && c <= '9':
			filtered = append(filtered, c)
		}
	}
	for left, right := 0, len(filtered)-1; left < right; left, right = left+1, right-1 {
		if filtered[left] != filtered[right] {
			return false
		}
	}
	return true
}
