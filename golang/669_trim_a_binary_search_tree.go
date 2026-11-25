package leetcode

// LeetCode #669 - Trim a Binary Search Tree
//
// A value below the range keeps its right subtree, and a value above the range keeps its left subtree.
func trimBST(root *TreeNode, low, high int) *TreeNode {
	if root == nil {
		return nil
	}
	if root.Val < low {
		return trimBST(root.Right, low, high)
	}
	if root.Val > high {
		return trimBST(root.Left, low, high)
	}
	root.Left = trimBST(root.Left, low, high)
	root.Right = trimBST(root.Right, low, high)
	return root
}
