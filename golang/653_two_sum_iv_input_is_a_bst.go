package leetcode

// LeetCode #653 - Two Sum IV - Input is a BST
//
// Each value is remembered, and a later node completes the sum when its partner was already seen.
func findTarget(root *TreeNode, k int) bool {
	seen := map[int]bool{}
	var walk func(*TreeNode) bool
	walk = func(node *TreeNode) bool {
		if node == nil {
			return false
		}
		if seen[k-node.Val] {
			return true
		}
		seen[node.Val] = true
		return walk(node.Left) || walk(node.Right)
	}
	return walk(root)
}
