package leetcode

// LeetCode #104 - Maximum Depth of Binary Tree
//
// Depth is one plus the deeper of the two subtrees.
func maxDepth(root *TreeNode) int {
	if root == nil {
		return 0
	}
	left := maxDepth(root.Left)
	right := maxDepth(root.Right)
	if left > right {
		return left + 1
	}
	return right + 1
}
