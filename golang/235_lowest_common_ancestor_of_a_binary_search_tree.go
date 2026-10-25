package leetcode

// LeetCode #235 - Lowest Common Ancestor of a Binary Search Tree
//
// Walk toward the side that can still contain both values.
func lowestCommonAncestor(root, p, q *TreeNode) *TreeNode {
	lo, hi := p.Val, q.Val
	if lo > hi {
		lo, hi = hi, lo
	}
	for root != nil {
		if root.Val < lo {
			root = root.Right
		} else if root.Val > hi {
			root = root.Left
		} else {
			return root
		}
	}
	return nil
}
