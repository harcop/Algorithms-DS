package leetcode

// LeetCode #103 - Binary Tree Zigzag Level Order Traversal
//
// Collect each level left to right, then reverse every other level.
func zigzagLevelOrder(root *TreeNode) [][]int {
	out := levelOrder(root)
	for i := range out {
		if i%2 == 1 {
			for left, right := 0, len(out[i])-1; left < right; left, right = left+1, right-1 {
				out[i][left], out[i][right] = out[i][right], out[i][left]
			}
		}
	}
	return out
}
