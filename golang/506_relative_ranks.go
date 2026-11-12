package leetcode

import (
	"sort"
	"strconv"
)

// LeetCode #506 - Relative Ranks
//
// Sort the scores, then the first three places become medals and the rest keep their place number.
func findRelativeRanks(score []int) []string {
	order := make([]int, len(score))
	for i := range order {
		order[i] = i
	}
	sort.Slice(order, func(i, j int) bool {
		return score[order[i]] > score[order[j]]
	})
	out := make([]string, len(score))
	for rank, index := range order {
		switch rank {
		case 0:
			out[index] = "Gold Medal"
		case 1:
			out[index] = "Silver Medal"
		case 2:
			out[index] = "Bronze Medal"
		default:
			out[index] = strconv.Itoa(rank + 1)
		}
	}
	return out
}
