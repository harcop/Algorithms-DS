package leetcode

// LeetCode #140 - Word Break II
//
// LeetCode names this wordBreak; that name already belongs to the yes-or-no version (#139).
// Remember every sentence that segments each suffix.
func wordBreakII(s string, wordDict []string) []string {
	memo := map[int][]string{}

	var dfs func(start int) []string
	dfs = func(start int) []string {
		if sentences, ok := memo[start]; ok {
			return sentences
		}
		if start == len(s) {
			return []string{""}
		}
		var sentences []string
		for _, word := range wordDict {
			end := start + len(word)
			if end > len(s) || s[start:end] != word {
				continue
			}
			for _, tail := range dfs(end) {
				if tail == "" {
					sentences = append(sentences, word)
				} else {
					sentences = append(sentences, word+" "+tail)
				}
			}
		}
		memo[start] = sentences
		return sentences
	}

	var out []string
	for _, sentence := range dfs(0) {
		if sentence != "" {
			out = append(out, sentence)
		}
	}
	return out
}
