package leetcode

import "sort"

// LeetCode #462 - Minimum Moves to Equal Array Elements II
//
// Every element moves to the median, and the cost is the total distance from that median.
func minMoves2(nums []int) int {
	sort.Ints(nums)
	mid := nums[len(nums)/2]
	moves := int64(0)
	for _, value := range nums {
		moves += int64(absInt(value - mid))
	}
	return int(moves)
}
