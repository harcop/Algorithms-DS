package leetcode

// LeetCode #472 - Concatenated Words
//
// A word qualifies when it can be cut into at least two shorter dictionary words.
func findAllConcatenatedWordsInADict(words []string) []string {
	dict := map[string]bool{}
	for _, word := range words {
		if word != "" {
			dict[word] = true
		}
	}
	var out []string
	for _, word := range words {
		if word != "" && concatenatedWord(word, dict) {
			out = append(out, word)
		}
	}
	return out
}

func concatenatedWord(word string, dict map[string]bool) bool {
	memo := make([]int, len(word)+1)
	var search func(start int) bool
	search = func(start int) bool {
		if start == len(word) {
			return true
		}
		if memo[start] != 0 {
			return memo[start] == 1
		}
		for end := start + 1; end <= len(word); end++ {
			if start == 0 && end == len(word) {
				continue
			}
			if !dict[word[start:end]] {
				continue
			}
			if search(end) {
				memo[start] = 1
				return true
			}
		}
		memo[start] = 2
		return false
	}
	return search(0)
}
