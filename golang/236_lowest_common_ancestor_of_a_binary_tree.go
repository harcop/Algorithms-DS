package leetcode

// LeetCode #236 - Lowest Common Ancestor of a Binary Tree
//
// LeetCode names this lowestCommonAncestor; that name already belongs to the BST version (#235).
// The ancestor is the first node whose subtrees together cover both targets.
func lowestCommonAncestorII(root *TreeNode, p, q int) int {
	var walk func(*TreeNode) (int, bool)
	walk = func(node *TreeNode) (int, bool) {
		if node == nil {
			return 0, false
		}
		left, leftOK := walk(node.Left)
		right, rightOK := walk(node.Right)
		if leftOK && rightOK {
			return node.Val, true
		}
		if node.Val == p || node.Val == q {
			return node.Val, true
		}
		if leftOK {
			return left, true
		}
		return right, rightOK
	}
	value, _ := walk(root)
	return value
}
