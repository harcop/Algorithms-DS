package leetcode

// LeetCode #555 - Split Concatenated Strings
//
// Each string may be reversed, and the split can start at any character of one string.
func splitLoopedString(strs []string) string {
	best := make([]string, len(strs))
	for i, word := range strs {
		reversed := reverseWord(word)
		if word >= reversed {
			best[i] = word
			continue
		}
		best[i] = reversed
	}
	answer := ""
	for i, word := range strs {
		middle := ""
		for _, part := range best[i+1:] {
			middle += part
		}
		for _, part := range best[:i] {
			middle += part
		}
		for _, body := range []string{word, reverseWord(word)} {
			for cut := 0; cut < len(body); cut++ {
				candidate := body[cut:] + middle + body[:cut]
				if candidate > answer {
					answer = candidate
				}
			}
		}
	}
	return answer
}

func reverseWord(word string) string {
	chars := []byte(word)
	for left, right := 0, len(chars)-1; left < right; left, right = left+1, right-1 {
		chars[left], chars[right] = chars[right], chars[left]
	}
	return string(chars)
}
