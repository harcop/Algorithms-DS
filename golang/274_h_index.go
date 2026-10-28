package leetcode

import "sort"

// LeetCode #274 - H-Index
//
// After sorting from most cited, h is the largest count that still has at least that many citations.
func hIndex(citations []int) int {
	sort.Slice(citations, func(i, j int) bool {
		return citations[i] > citations[j]
	})
	h := 0
	for i, count := range citations {
		if count >= i+1 {
			h = i + 1
			continue
		}
		break
	}
	return h
}
