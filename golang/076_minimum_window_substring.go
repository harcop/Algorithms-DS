package leetcode

// LeetCode #76 - Minimum Window Substring
//
// Expand the window until it covers t, then shrink from the left and keep the shortest cover.
func minWindow(s string, t string) string {
	if len(s) < len(t) || t == "" {
		return ""
	}

	need := make(map[byte]int, len(t))
	for i := 0; i < len(t); i++ {
		need[t[i]]++
	}
	required := len(need)
	window := make(map[byte]int)
	have := 0
	bestLeft, bestLen := 0, len(s)+1
	left := 0

	for right := 0; right < len(s); right++ {
		c := s[right]
		window[c]++
		if count, ok := need[c]; ok && window[c] == count {
			have++
		}

		for have == required && left <= right {
			if right-left+1 < bestLen {
				bestLeft = left
				bestLen = right - left + 1
			}
			leftChar := s[left]
			if count, ok := need[leftChar]; ok && window[leftChar] == count {
				have--
			}
			window[leftChar]--
			left++
		}
	}

	if bestLen == len(s)+1 {
		return ""
	}
	return s[bestLeft : bestLeft+bestLen]
}
