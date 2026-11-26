package leetcode

import "sort"

// LeetCode #692 - Top K Frequent Words
//
// LeetCode names this topKFrequent; that name already belongs to the number version (#347).
// Higher counts come first, and equal counts keep alphabetical order.
func topKFrequentWords(words []string, k int) []string {
	freq := map[string]int{}
	for _, word := range words {
		freq[word]++
	}
	unique := make([]string, 0, len(freq))
	for word := range freq {
		unique = append(unique, word)
	}
	sort.Slice(unique, func(i, j int) bool {
		if freq[unique[i]] == freq[unique[j]] {
			return unique[i] < unique[j]
		}
		return freq[unique[i]] > freq[unique[j]]
	})
	return unique[:k]
}
