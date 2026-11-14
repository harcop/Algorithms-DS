package leetcode

import "math"

// LeetCode #530 - Minimum Absolute Difference in BST
//
// An inorder walk visits the values in order, so the minimum gap is between some pair of neighbors.
func getMinimumDifference(root *TreeNode) int {
	previous := -1
	best := math.MaxInt
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		if previous >= 0 {
			best = min(best, node.Val-previous)
		}
		previous = node.Val
		walk(node.Right)
	}
	walk(root)
	return best
}
