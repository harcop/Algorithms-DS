package leetcode

// LeetCode #687 - Longest Univalue Path
//
// An equal child extends an arm, and the best path is the two arms that meet at a node.
func longestUnivaluePath(root *TreeNode) int {
	best := 0
	var arm func(*TreeNode) int
	arm = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left, right := arm(node.Left), arm(node.Right)
		leftEdge, rightEdge := 0, 0
		if node.Left != nil && node.Left.Val == node.Val {
			leftEdge = left + 1
		}
		if node.Right != nil && node.Right.Val == node.Val {
			rightEdge = right + 1
		}
		best = max(best, leftEdge+rightEdge)
		return max(leftEdge, rightEdge)
	}
	arm(root)
	return best
}
