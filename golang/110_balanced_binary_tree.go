package leetcode

// LeetCode #110 - Balanced Binary Tree
//
// A tree is balanced when every subtree is balanced and the child heights differ by at most one.
func isBalanced(root *TreeNode) bool {
	var height func(*TreeNode) int
	height = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := height(node.Left)
		if left < 0 {
			return -1
		}
		right := height(node.Right)
		if right < 0 {
			return -1
		}
		diff := left - right
		if diff < 0 {
			diff = -diff
		}
		if diff > 1 {
			return -1
		}
		if left > right {
			return left + 1
		}
		return right + 1
	}
	return height(root) >= 0
}
