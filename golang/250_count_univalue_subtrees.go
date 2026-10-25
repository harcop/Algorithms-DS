package leetcode

// LeetCode #250 - Count Univalue Subtrees
//
// A subtree counts when it and both children are univalue and share this node's value.
func countUnivalSubtrees(root *TreeNode) int {
	count := 0
	var univalue func(*TreeNode) bool
	univalue = func(node *TreeNode) bool {
		if node == nil {
			return true
		}
		left := univalue(node.Left)
		right := univalue(node.Right)
		same := left && right
		if node.Left != nil && node.Left.Val != node.Val {
			same = false
		}
		if node.Right != nil && node.Right.Val != node.Val {
			same = false
		}
		if same {
			count++
		}
		return same
	}
	univalue(root)
	return count
}
