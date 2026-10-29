package leetcode

import "sort"

// LeetCode #300 - Longest Increasing Subsequence
//
// Keep the smallest tail of every increasing length seen so far.
func lengthOfLIS(nums []int) int {
	var tails []int
	for _, value := range nums {
		at := sort.Search(len(tails), func(i int) bool { return tails[i] >= value })
		if at == len(tails) {
			tails = append(tails, value)
		} else {
			tails[at] = value
		}
	}
	return len(tails)
}
