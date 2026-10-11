package leetcode

// LeetCode #9 - Palindrome Number
//
// Return true if x reads the same forward and backward. Negative numbers are not palindromes.
func isPalindrome(x int) bool {
	if x < 0 {
		return false
	}
	if x != 0 && x%10 == 0 {
		return false
	}

	n := x
	revHalf := 0

	for n > revHalf {
		revHalf = revHalf*10 + n%10
		n /= 10
	}

	return n == revHalf || n == revHalf/10
}
