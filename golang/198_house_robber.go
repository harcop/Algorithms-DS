package leetcode

// LeetCode #198 - House Robber
//
// The best haul is either the previous house or this house plus the one before that.
func rob(nums []int) int {
	skip, take := 0, 0
	for _, value := range nums {
		skip, take = max(skip, take), skip+value
	}
	return max(skip, take)
}
