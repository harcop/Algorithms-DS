package leetcode

import "math"

// LeetCode #333 - Largest BST Subtree
//
// A subtree is a BST when both children are and the node sits strictly between their ranges.
func largestBSTSubtree(root *TreeNode) int {
	best := 0
	var walk func(*TreeNode) (int64, int64, int)
	walk = func(node *TreeNode) (int64, int64, int) {
		if node == nil {
			return math.MaxInt64, math.MinInt64, 0
		}
		leftMin, leftMax, leftSize := walk(node.Left)
		rightMin, rightMax, rightSize := walk(node.Right)
		value := int64(node.Val)
		if leftSize >= 0 && rightSize >= 0 && value > leftMax && value < rightMin {
			size := leftSize + rightSize + 1
			if size > best {
				best = size
			}
			if value < leftMin {
				leftMin = value
			}
			if value > rightMax {
				rightMax = value
			}
			return leftMin, rightMax, size
		}
		return -1, -1, -1
	}
	walk(root)
	return best
}
