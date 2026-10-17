package leetcode

// LeetCode #112 - Path Sum
//
// Subtract each node from the remaining sum and accept a leaf only when nothing is left.
func hasPathSum(root *TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}
	remain := targetSum - root.Val
	if root.Left == nil && root.Right == nil {
		return remain == 0
	}
	return hasPathSum(root.Left, remain) || hasPathSum(root.Right, remain)
}
