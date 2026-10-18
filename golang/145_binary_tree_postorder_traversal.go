package leetcode

// LeetCode #145 - Binary Tree Postorder Traversal
//
// Visit the left subtree, then the right subtree, then the node.
func postorderTraversal(root *TreeNode) []int {
	var out []int
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		walk(node.Right)
		out = append(out, node.Val)
	}
	walk(root)
	return out
}
