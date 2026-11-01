package leetcode

// LeetCode #340 - Longest Substring with At Most K Distinct Characters
//
// Expand a window and shrink it from the left whenever it holds more than k characters.
func lengthOfLongestSubstringKDistinct(s string, k int) int {
	freq := map[byte]int{}
	left, best := 0, 0
	for right := 0; right < len(s); right++ {
		freq[s[right]]++
		for len(freq) > k {
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
