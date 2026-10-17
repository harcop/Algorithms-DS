package leetcode

import "math"

// LeetCode #124 - Binary Tree Maximum Path Sum
//
// A path through a node uses the positive gain from each side. Return only one side upward.
func maxPathSum(root *TreeNode) int {
	best := math.MinInt
	var gain func(*TreeNode) int
	gain = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := gain(node.Left)
		if left < 0 {
			left = 0
		}
		right := gain(node.Right)
		if right < 0 {
			right = 0
		}
		if through := node.Val + left + right; through > best {
			best = through
		}
		if left > right {
			return node.Val + left
		}
		return node.Val + right
	}
	gain(root)
	return best
}
