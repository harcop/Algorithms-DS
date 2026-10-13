package leetcode

// LeetCode #30 - Substring with Concatenation of All Words
//
// Slide a window of word-sized chunks for each alignment and match the required word counts.
func findSubstring(s string, words []string) []int {
	if len(words) == 0 {
		return nil
	}

	wordLen := len(words[0])
	wordCount := len(words)
	totalLen := wordLen * wordCount
	if wordLen == 0 || len(s) < totalLen {
		return nil
	}

	target := make(map[string]int, wordCount)
	for _, word := range words {
		target[word]++
	}

	var result []int
	for offset := 0; offset < wordLen; offset++ {
		left := offset
		seen := make(map[string]int)
		used := 0

		for right := offset; right+wordLen <= len(s); right += wordLen {
			word := s[right : right+wordLen]
			if _, ok := target[word]; !ok {
				seen = make(map[string]int)
				used = 0
				left = right + wordLen
				continue
			}

			seen[word]++
			used++
			for seen[word] > target[word] {
				leftWord := s[left : left+wordLen]
				seen[leftWord]--
				left += wordLen
				used--
			}
			if used == wordCount {
				result = append(result, left)
				leftWord := s[left : left+wordLen]
				seen[leftWord]--
				left += wordLen
				used--
			}
		}
	}

	return result
}
