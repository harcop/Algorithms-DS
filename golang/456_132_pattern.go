package leetcode

import "math"

// LeetCode #456 - 132 Pattern
//
// Scan from the right and keep the best "3" and "2" candidates on a decreasing stack.
func find132pattern(nums []int) bool {
	stack := make([]int, 0, len(nums))
	third := math.MinInt
	for i := len(nums) - 1; i >= 0; i-- {
		if nums[i] < third {
			return true
		}
		for len(stack) > 0 && nums[i] > stack[len(stack)-1] {
			third = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, nums[i])
	}
	return false
}
