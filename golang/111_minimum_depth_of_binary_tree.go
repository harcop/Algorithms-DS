package leetcode

// LeetCode #111 - Minimum Depth of Binary Tree
//
// A missing child is not a shorter path. Only a node with no children ends the depth.
func minDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	if root.Left == nil && root.Right == nil {
		return 1
	}
	if root.Left == nil {
		return 1 + minDepth(root.Right)
	}
	if root.Right == nil {
		return 1 + minDepth(root.Left)
	}
	left, right := minDepth(root.Left), minDepth(root.Right)
	if left < right {
		return 1 + left
	}
	return 1 + right
}
