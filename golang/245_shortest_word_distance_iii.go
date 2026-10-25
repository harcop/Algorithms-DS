package leetcode

// LeetCode #245 - Shortest Word Distance III
//
// Same scan as the first distance problem, including the distance between two copies of one word.
func shortestWordDistance(wordsDict []string, word1 string, word2 string) int {
	best := len(wordsDict)
	if word1 == word2 {
		prev := -1
		for i, word := range wordsDict {
			if word != word1 {
				continue
			}
			if prev >= 0 && i-prev < best {
				best = i - prev
			}
			prev = i
		}
		return best
	}
	i1, i2 := -1, -1
	for i, word := range wordsDict {
		if word == word1 {
			i1 = i
		}
		if word == word2 {
			i2 = i
		}
		if i1 >= 0 && i2 >= 0 {
			gap := i1 - i2
			if gap < 0 {
				gap = -gap
			}
			if gap < best {
				best = gap
			}
		}
	}
	return best
}
