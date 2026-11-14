package leetcode

// LeetCode #538 - Convert BST to Greater Tree
//
// Visit the larger values first and replace each node with the sum of every value seen so far.
func convertBST(root *TreeNode) *TreeNode {
	sum := 0
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Right)
		sum += node.Val
		node.Val = sum
		walk(node.Left)
	}
	walk(root)
	return root
}
