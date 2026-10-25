package leetcode

import "math"

// LeetCode #255 - Verify Preorder Sequence in Binary Search Tree
//
// A value smaller than the last ancestor we moved right from cannot belong in this tree.
func verifyPreorder(preorder []int) bool {
	var stack []int
	lower := math.MinInt32
	for _, value := range preorder {
		if value < lower {
			return false
		}
		for len(stack) > 0 && value > stack[len(stack)-1] {
			lower = stack[len(stack)-1]
			stack = stack[:len(stack)-1]
		}
		stack = append(stack, value)
	}
	return true
}
