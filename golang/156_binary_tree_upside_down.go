package leetcode

// LeetCode #156 - Binary Tree Upside Down
//
// Walk down the left spine and make each node the parent of its old parent and right sibling.
func upsideDownBinaryTree(root *TreeNode) *TreeNode {
	var prev, rightSibling *TreeNode
	for root != nil {
		next := root.Left
		oldRight := root.Right
		root.Left = rightSibling
		root.Right = prev
		prev = root
		rightSibling = oldRight
		root = next
	}
	return prev
}
