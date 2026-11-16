package leetcode

// LeetCode #545 - Boundary of Binary Tree
//
// Walk the left edge downward, then the leaves from left to right, then the right edge upward.
func boundaryOfBinaryTree(root *TreeNode) []int {
	if root == nil {
		return nil
	}
	if root.Left == nil && root.Right == nil {
		return []int{root.Val}
	}
	out := []int{root.Val}
	var leftEdge func(*TreeNode)
	leftEdge = func(node *TreeNode) {
		if node == nil || (node.Left == nil && node.Right == nil) {
			return
		}
		out = append(out, node.Val)
		if node.Left != nil {
			leftEdge(node.Left)
			return
		}
		leftEdge(node.Right)
	}
	var leaves func(*TreeNode)
	leaves = func(node *TreeNode) {
		if node == nil {
			return
		}
		if node.Left == nil && node.Right == nil {
			out = append(out, node.Val)
			return
		}
		leaves(node.Left)
		leaves(node.Right)
	}
	var rightEdge func(*TreeNode)
	rightEdge = func(node *TreeNode) {
		if node == nil || (node.Left == nil && node.Right == nil) {
			return
		}
		if node.Right != nil {
			rightEdge(node.Right)
		} else {
			rightEdge(node.Left)
		}
		out = append(out, node.Val)
	}
	leftEdge(root.Left)
	leaves(root)
	rightEdge(root.Right)
	return out
}
