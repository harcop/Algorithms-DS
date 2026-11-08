package leetcode

// LeetCode #450 - Delete Node in a BST
//
// LeetCode names this deleteNode; that name already belongs to the linked-list overwrite (#237).
// A node with two children is replaced by its inorder successor, which is then removed from the right subtree.
func deleteBSTNode(root *TreeNode, key int) *TreeNode {
	if root == nil {
		return nil
	}
	if key < root.Val {
		root.Left = deleteBSTNode(root.Left, key)
		return root
	}
	if key > root.Val {
		root.Right = deleteBSTNode(root.Right, key)
		return root
	}
	if root.Left == nil {
		return root.Right
	}
	if root.Right == nil {
		return root.Left
	}
	successor := root.Right
	for successor.Left != nil {
		successor = successor.Left
	}
	root.Val = successor.Val
	root.Right = deleteBSTNode(root.Right, successor.Val)
	return root
}
