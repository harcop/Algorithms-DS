package leetcode

// LeetCode #242 - Valid Anagram
//
// The two strings are anagrams when every letter count cancels out.
func isAnagram(s string, t string) bool {
	if len(s) != len(t) {
		return false
	}
	var count [26]int
	for i := 0; i < len(s); i++ {
		count[s[i]-'a']++
		count[t[i]-'a']--
	}
	for _, n := range count {
		if n != 0 {
			return false
		}
	}
	return true
}
