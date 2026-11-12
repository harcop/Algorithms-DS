package leetcode

// ParentTreeNode is a binary search tree node that also points at its parent.
type ParentTreeNode struct {
	Val    int
	Left   *ParentTreeNode
	Right  *ParentTreeNode
	Parent *ParentTreeNode
}

// LeetCode #510 - Inorder Successor in BST II
//
// The successor is the leftmost node of the right subtree, or the first ancestor reached from a left child.
func inorderSuccessorII(node *ParentTreeNode) *ParentTreeNode {
	if node == nil {
		return nil
	}
	if node.Right != nil {
		node = node.Right
		for node.Left != nil {
			node = node.Left
		}
		return node
	}
	for node.Parent != nil && node == node.Parent.Right {
		node = node.Parent
	}
	return node.Parent
}
