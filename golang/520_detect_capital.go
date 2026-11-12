package leetcode

// LeetCode #520 - Detect Capital
//
// The word is valid when every letter is capital, none are, or only the first one is.
func detectCapitalUse(word string) bool {
	upper := 0
	for i := 0; i < len(word); i++ {
		if word[i] >= 'A' && word[i] <= 'Z' {
			upper++
		}
	}
	return upper == 0 || upper == len(word) || (upper == 1 && word[0] >= 'A' && word[0] <= 'Z')
}
