package leetcode

import "sort"

// LeetCode #354 - Russian Doll Envelopes
//
// Sort by width, then take the longest increasing height with a patience sort.
func maxEnvelopes(envelopes [][]int) int {
	sort.Slice(envelopes, func(i, j int) bool {
		if envelopes[i][0] == envelopes[j][0] {
			return envelopes[i][1] > envelopes[j][1]
		}
		return envelopes[i][0] < envelopes[j][0]
	})
	tails := make([]int, 0, len(envelopes))
	for _, envelope := range envelopes {
		height := envelope[1]
		at := sort.Search(len(tails), func(i int) bool { return tails[i] >= height })
		if at == len(tails) {
			tails = append(tails, height)
		} else {
			tails[at] = height
		}
	}
	return len(tails)
}
