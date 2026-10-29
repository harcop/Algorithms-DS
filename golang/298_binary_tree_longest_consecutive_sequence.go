package leetcode

// LeetCode #298 - Binary Tree Longest Consecutive Sequence
//
// LeetCode names this longestConsecutive; that name already belongs to the array version (#128).
// Extend a run when a child is exactly one larger than its parent.
func longestConsecutiveTree(root *TreeNode) int {
	best := 0
	var walk func(node *TreeNode, prev, length int, hasPrev bool)
	walk = func(node *TreeNode, prev, length int, hasPrev bool) {
		if node == nil {
			return
		}
		next := 1
		if hasPrev && node.Val == prev+1 {
			next = length + 1
		}
		if next > best {
			best = next
		}
		walk(node.Left, node.Val, next, true)
		walk(node.Right, node.Val, next, true)
	}
	walk(root, 0, 0, false)
	return best
}
