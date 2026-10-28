package leetcode

// LeetCode #285 - Inorder Successor in BST
//
// The successor is the smallest value still larger than p on the search path.
func inorderSuccessor(root *TreeNode, p int) int {
	successor := -1
	for root != nil {
		if root.Val > p {
			successor = root.Val
			root = root.Left
		} else {
			root = root.Right
		}
	}
	return successor
}
