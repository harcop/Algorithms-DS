package leetcode

import "sort"

// LeetCode #646 - Maximum Length of Pair Chain
//
// Taking the pair that ends first leaves the most room for a later pair to follow it.
func findLongestChain(pairs [][]int) int {
	sort.Slice(pairs, func(i, j int) bool {
		return pairs[i][1] < pairs[j][1]
	})
	length := 0
	end := 0
	started := false
	for _, pair := range pairs {
		if !started || pair[0] > end {
			length++
			end = pair[1]
			started = true
		}
	}
	return length
}
