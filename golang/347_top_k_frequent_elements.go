package leetcode

import "sort"

// LeetCode #347 - Top K Frequent Elements
//
// Count each value, then keep the k counts that appear most often.
func topKFrequent(nums []int, k int) []int {
	counts := map[int]int{}
	for _, value := range nums {
		counts[value]++
	}
	ranked := make([][2]int, 0, len(counts))
	for value, count := range counts {
		ranked = append(ranked, [2]int{value, count})
	}
	sort.Slice(ranked, func(i, j int) bool { return ranked[i][1] > ranked[j][1] })
	out := make([]int, k)
	for i := 0; i < k; i++ {
		out[i] = ranked[i][0]
	}
	return out
}
