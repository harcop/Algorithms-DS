package leetcode

// LeetCode #671 - Second Minimum Node In a Binary Tree
//
// A child larger than the root is a candidate, and a child equal to the root defers to its own subtree.
func findSecondMinimumValue(root *TreeNode) int {
	if root == nil || root.Left == nil {
		return -1
	}
	left, right := root.Left.Val, root.Right.Val
	if left == root.Val {
		left = findSecondMinimumValue(root.Left)
	}
	if right == root.Val {
		right = findSecondMinimumValue(root.Right)
	}
	if left == -1 {
		return right
	}
	if right == -1 {
		return left
	}
	return min(left, right)
}
