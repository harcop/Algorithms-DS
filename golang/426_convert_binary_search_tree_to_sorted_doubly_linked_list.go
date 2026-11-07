package leetcode

// LeetCode #426 - Convert Binary Search Tree to Sorted Doubly Linked List
//
// An inorder walk links each node to its predecessor and successor, then closes the circle.
// Left is the previous node and Right is the next node.
func treeToDoublyList(root *TreeNode) *TreeNode {
	if root == nil {
		return nil
	}
	var first, last *TreeNode
	var walk func(*TreeNode)
	walk = func(node *TreeNode) {
		if node == nil {
			return
		}
		walk(node.Left)
		if last == nil {
			first = node
		} else {
			last.Right = node
			node.Left = last
		}
		last = node
		walk(node.Right)
	}
	walk(root)
	first.Left = last
	last.Right = first
	return first
}
