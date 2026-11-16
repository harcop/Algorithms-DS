package leetcode

// LeetCode #543 - Diameter of Binary Tree
//
// The diameter through a node is the sum of its two downward heights, and the answer is the largest of those.
func diameterOfBinaryTree(root *TreeNode) int {
	best := 0
	var height func(*TreeNode) int
	height = func(node *TreeNode) int {
		if node == nil {
			return 0
		}
		left := height(node.Left)
		right := height(node.Right)
		best = max(best, left+right)
		return max(left, right) + 1
	}
	height(root)
	return best
}
