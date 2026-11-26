package leetcode

// LeetCode #700 - Search in a Binary Search Tree
//
// Smaller values live on the left and larger values live on the right, until the target node is found.
func searchBST(root *TreeNode, val int) *TreeNode {
	for root != nil && root.Val != val {
		if val < root.Val {
			root = root.Left
			continue
		}
		root = root.Right
	}
	return root
}
