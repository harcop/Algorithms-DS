package leetcode

// LeetCode #549 - Binary Tree Longest Consecutive Sequence II
//
// LeetCode names this longestConsecutive; that name already belongs to the array version (#128).
// Each node keeps the longest increasing and decreasing chains below it, and a path may use one of each.
func longestConsecutiveEither(root *TreeNode) int {
	best := 0
	var walk func(*TreeNode) (inc, dec int)
	walk = func(node *TreeNode) (int, int) {
		if node == nil {
			return 0, 0
		}
		leftInc, leftDec := walk(node.Left)
		rightInc, rightDec := walk(node.Right)
		inc, dec := 1, 1
		if node.Left != nil && node.Left.Val == node.Val+1 {
			inc = max(inc, leftInc+1)
		}
		if node.Left != nil && node.Left.Val == node.Val-1 {
			dec = max(dec, leftDec+1)
		}
		if node.Right != nil && node.Right.Val == node.Val+1 {
			inc = max(inc, rightInc+1)
		}
		if node.Right != nil && node.Right.Val == node.Val-1 {
			dec = max(dec, rightDec+1)
		}
		best = max(best, inc+dec-1)
		return inc, dec
	}
	walk(root)
	return best
}
