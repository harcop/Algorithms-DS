package leetcode

// LeetCode #572 - Subtree of Another Tree
//
// The subtree matches when some node is the root of an identical tree.
func isSubtree(root *TreeNode, subRoot *TreeNode) bool {
	if subRoot == nil {
		return true
	}
	if root == nil {
		return false
	}
	return isSameTree(root, subRoot) || isSubtree(root.Left, subRoot) || isSubtree(root.Right, subRoot)
}
