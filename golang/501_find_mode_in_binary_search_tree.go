package leetcode

// LeetCode #501 - Find Mode in Binary Search Tree
//
// An inorder walk counts equal neighbors, and every value that matches the highest count is a mode.
func findMode(root *TreeNode) []int {
	var modes []int
	prev, count, maxCount := 0, 0, 0
	hasPrev := false
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		if hasPrev && prev == node.Val {
			count++
		} else {
			count = 1
		}
		if count > maxCount {
			maxCount = count
			modes = []int{node.Val}
		} else if count == maxCount {
			modes = append(modes, node.Val)
		}
		prev = node.Val
		hasPrev = true
		walk(node.Right)
	}
	walk(root)
	return modes
}
