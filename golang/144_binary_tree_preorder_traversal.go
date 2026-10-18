package leetcode

// LeetCode #144 - Binary Tree Preorder Traversal
//
// Visit the node, then the left subtree, then the right subtree.
func preorderTraversal(root *TreeNode) []int {
	var out []int
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		out = append(out, node.Val)
		walk(node.Left)
		walk(node.Right)
	}
	walk(root)
	return out
}
