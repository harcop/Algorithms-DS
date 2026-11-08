package leetcode

// LeetCode #453 - Minimum Moves to Equal Array Elements
//
// Incrementing every element but one is the same as decrementing that one, so the cost is the distance above the minimum.
func minMoves(nums []int) int {
	low := nums[0]
	for _, value := range nums[1:] {
		if value < low {
			low = value
		}
	}
	moves := int64(0)
	for _, value := range nums {
		moves += int64(value - low)
	}
	return int(moves)
}
