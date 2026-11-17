package leetcode

// LeetCode #563 - Binary Tree Tilt
//
// A node's tilt is the absolute gap between its subtree sums, and the answer adds every tilt.
func findTilt(root *TreeNode) int {
	total := 0
	var subtreeSum func(*TreeNode) int
	subtreeSum = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := subtreeSum(node.Left)
		right := subtreeSum(node.Right)
		gap := left - right
		if gap < 0 {
			gap = -gap
		}
		total += gap
		return left + right + node.Val
	}
	subtreeSum(root)
	return total
}
