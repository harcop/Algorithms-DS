package leetcode

// LeetCode #222 - Count Complete Tree Nodes
//
// Every node adds one, plus the sizes of its two subtrees.
func countNodes(root *TreeNode) int {
	if root == nil {
		return 0
	}
	return 1 + countNodes(root.Left) + countNodes(root.Right)
}
