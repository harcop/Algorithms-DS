package leetcode

// LeetCode #139 - Word Break
//
// A prefix can be segmented when some dictionary word ends there and the earlier prefix can too.
func wordBreak(s string, wordDict []string) bool {
	dict := make(map[string]struct{}, len(wordDict))
	for _, word := range wordDict {
		dict[word] = struct{}{}
	}
	dp := make([]bool, len(s)+1)
	dp[0] = true
	for end := 1; end <= len(s); end++ {
		for start := 0; start < end; start++ {
			if dp[start] {
				if _, ok := dict[s[start:end]]; ok {
					dp[end] = true
					break
				}
			}
		}
	}
	return dp[len(s)]
}
