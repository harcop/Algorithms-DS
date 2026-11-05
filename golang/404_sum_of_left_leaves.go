package leetcode

// LeetCode #404 - Sum of Left Leaves
//
// Add a leaf only when the edge that reached it came from its parent's left side.
func sumOfLeftLeaves(root *TreeNode) int {
	var walk func(node *TreeNode, leftChild bool) int
	walk = func(node *TreeNode, leftChild bool) int {
		if node == nil {
			return 0
		}
		if node.Left == nil && node.Right == nil && leftChild {
			return node.Val
		}
		return walk(node.Left, true) + walk(node.Right, false)
	}
	return walk(root, false)
}
