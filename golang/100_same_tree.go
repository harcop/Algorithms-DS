package leetcode

// LeetCode #100 - Same Tree
//
// Both nodes are missing, or they hold the same value and their children match.
func isSameTree(p *TreeNode, q *TreeNode) bool {
	if p == nil || q == nil {
		return p == q
	}
	return p.Val == q.Val && isSameTree(p.Left, q.Left) && isSameTree(p.Right, q.Right)
}
