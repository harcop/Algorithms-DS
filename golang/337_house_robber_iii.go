package leetcode

// LeetCode #337 - House Robber III
//
// LeetCode names this rob; that name already belongs to the linear street (#198).
// Each house returns the best loot if it is taken and if it is skipped.
func robIII(root *TreeNode) int {
	var loot func(*TreeNode) (take, skip int)
	loot = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}
		leftTake, leftSkip := loot(node.Left)
		rightTake, rightSkip := loot(node.Right)
		take := node.Val + leftSkip + rightSkip
		skip := max(leftTake, leftSkip) + max(rightTake, rightSkip)
		return take, skip
	}
	take, skip := loot(root)
	return max(take, skip)
}
