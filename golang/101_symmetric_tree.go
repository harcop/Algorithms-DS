package leetcode

// LeetCode #101 - Symmetric Tree
//
// The left child mirrors the right child: outer children match, and inner children match.
func isSymmetric(root *TreeNode) bool {
	if root == nil {
		return true
	}
	var mirror func(a, b *TreeNode) bool
	mirror = func(a, b *TreeNode) bool {
		if a == nil || b == nil {
			return a == b
		}
		return a.Val == b.Val && mirror(a.Left, b.Right) && mirror(a.Right, b.Left)
	}
	return mirror(root.Left, root.Right)
}
