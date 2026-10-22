package leetcode

// LeetCode #230 - Kth Smallest Element in a BST
//
// Inorder visits the values in order, so the kth visit is the answer.
func kthSmallest(root *TreeNode, k int) int {
	var stack []*TreeNode
	for {
		for root != nil {
			stack = append(stack, root)
			root = root.Left
		}
		node := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		k--
		if k == 0 {
			return node.Val
		}
		root = node.Right
	}
}
