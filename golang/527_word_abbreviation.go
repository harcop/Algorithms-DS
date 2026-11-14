package leetcode

import "strconv"

// LeetCode #527 - Word Abbreviation
//
// Share a prefix only while several words still shorten to the same abbreviation.
func wordsAbbreviation(words []string) []string {
	n := len(words)
	prefix := make([]int, n)
	for i := range prefix {
		prefix[i] = 1
	}
	answer := make([]string, n)
	resolved := make([]bool, n)
	remaining := n
	for remaining > 0 {
		groups := map[string][]int{}
		for i := range words {
			if resolved[i] {
				continue
			}
			short := abbreviatePrefix(words[i], prefix[i])
			groups[short] = append(groups[short], i)
		}
		for short, indexes := range groups {
			if len(indexes) == 1 {
				i := indexes[0]
				answer[i] = short
				resolved[i] = true
				remaining--
				continue
			}
			for _, i := range indexes {
				if prefix[i] > len(words[i]) {
					answer[i] = words[i]
					resolved[i] = true
					remaining--
					continue
				}
				prefix[i]++
			}
		}
	}
	return answer
}

func abbreviatePrefix(word string, prefixLen int) string {
	n := len(word)
	if prefixLen >= n-1 {
		return word
	}
	skipped := n - prefixLen - 1
	short := word[:prefixLen] + strconv.Itoa(skipped) + word[n-1:]
	if len(short) >= n {
		return word
	}
	return short
}
