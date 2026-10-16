package leetcode

import "sort"

// LeetCode #99 - Recover Binary Search Tree
//
// Write the inorder values back in sorted order so the tree satisfies the BST property.
func recoverTree(root *TreeNode) {
	var nodes []*TreeNode
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		nodes = append(nodes, node)
		walk(node.Right)
	}
	walk(root)

	values := make([]int, len(nodes))
	for i, node := range nodes {
		values[i] = node.Val
	}
	sort.Ints(values)
	for i, node := range nodes {
		node.Val = values[i]
	}
}
