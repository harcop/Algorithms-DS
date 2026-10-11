package leetcode

// LeetCode #3 - Longest Substring Without Repeating Characters
//
// Returns the length of the longest substring without repeating characters.
func lengthOfLongestSubstring(s string) int {
	lastIndex := make(map[byte]int)
	windowStart := 0
	best := 0

	for i := 0; i < len(s); i++ {
		c := s[i]
		if prev, ok := lastIndex[c]; ok && prev+1 > windowStart {
			windowStart = prev + 1
		}
		lastIndex[c] = i
		if i-windowStart+1 > best {
			best = i - windowStart + 1
		}
	}

	return best
}
