package leetcode

// LeetCode #114 - Flatten Binary Tree to Linked List
//
// Splice each left subtree between a node and its right subtree, then clear the left link.
func flatten(root *TreeNode) {
	for curr := root; curr != nil; curr = curr.Right {
		if curr.Left == nil {
			continue
		}
		tail := curr.Left
		for tail.Right != nil {
			tail = tail.Right
		}
		tail.Right = curr.Right
		curr.Right = curr.Left
		curr.Left = nil
	}
}
