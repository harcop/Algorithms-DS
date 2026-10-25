package leetcode

// LeetCode #266 - Palindrome Permutation
//
// A palindrome can use at most one character an odd number of times.
func canPermutePalindrome(s string) bool {
	var odd [128]byte
	for i := 0; i < len(s); i++ {
		odd[s[i]] ^= 1
	}
	count := 0
	for _, bit := range odd {
		count += int(bit)
	}
	return count <= 1
}
