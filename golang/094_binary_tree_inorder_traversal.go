package leetcode

// TreeNode is a binary tree node.
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// LeetCode #94 - Binary Tree Inorder Traversal
//
// Visit the left subtree, then the node, then the right subtree.
func inorderTraversal(root *TreeNode) []int {
	var out []int
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		out = append(out, node.Val)
		walk(node.Right)
	}
	walk(root)
	return out
}
