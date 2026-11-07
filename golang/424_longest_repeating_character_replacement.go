package leetcode

// LeetCode #424 - Longest Repeating Character Replacement
//
// A window stays valid while k replacements cover every character except the most frequent one.
func characterReplacement(s string, k int) int {
	var count [26]int
	best, maxFreq, left := 0, 0, 0
	for right := 0; right < len(s); right++ {
		count[s[right]-'A']++
		if count[s[right]-'A'] > maxFreq {
			maxFreq = count[s[right]-'A']
		}
		for right-left+1-maxFreq > k {
			count[s[left]-'A']--
			left++
		}
		if right-left+1 > best {
			best = right - left + 1
		}
	}
	return best
}
