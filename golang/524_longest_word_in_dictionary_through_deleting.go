package leetcode

// LeetCode #524 - Longest Word in Dictionary through Deleting
//
// Keep the longest dictionary word that is a subsequence of s, and the lexicographically smaller one when lengths tie.
func findLongestWord(s string, dictionary []string) string {
	best := ""
	for _, word := range dictionary {
		if !isSubsequence(word, s) {
			continue
		}
		if len(word) > len(best) || (len(word) == len(best) && word < best) {
			best = word
		}
	}
	return best
}
