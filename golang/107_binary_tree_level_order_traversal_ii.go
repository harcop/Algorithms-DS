package leetcode

// LeetCode #107 - Binary Tree Level Order Traversal II
//
// Collect levels from the root downward, then reverse the list of levels.
func levelOrderBottom(root *TreeNode) [][]int {
	out := levelOrder(root)
	for i, j := 0, len(out)-1; i < j; i, j = i+1, j-1 {
		out[i], out[j] = out[j], out[i]
	}
	return out
}
