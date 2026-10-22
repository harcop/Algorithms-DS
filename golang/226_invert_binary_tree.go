package leetcode

// LeetCode #226 - Invert Binary Tree
//
// Swap the two children of every node.
func invertTree(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	root.Left, root.Right = invertTree(root.Right), invertTree(root.Left)
	return root
}
