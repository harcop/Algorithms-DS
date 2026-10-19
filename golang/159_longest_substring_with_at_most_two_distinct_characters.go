package leetcode

// LeetCode #159 - Longest Substring with At Most Two Distinct Characters
//
// Expand a window and shrink it from the left whenever a third character appears.
func lengthOfLongestSubstringTwoDistinct(s string) int {
	freq := map[byte]int{}
	left, best := 0, 0
	for right := 0; right < len(s); right++ {
		freq[s[right]]++
		for len(freq) > 2 {
			freq[s[left]]--
			if freq[s[left]] == 0 {
				delete(freq, s[left])
			}
			left++
		}
		if span := right - left + 1; span > best {
			best = span
		}
	}
	return best
}
