package leetcode

// Node is a binary tree node with a link to its right neighbor.
type Node struct {
	Val   int
	Left  *Node
	Right *Node
	Next  *Node
}

// LeetCode #116 - Populating Next Right Pointers in Each Node
//
// On a perfect tree, each left child points at its sibling and each right child points across.
func connect(root *Node) *Node {
	if root == nil {
		return nil
	}
	for leftmost := root; leftmost.Left != nil; leftmost = leftmost.Left {
		for node := leftmost; node != nil; node = node.Next {
			node.Left.Next = node.Right
			if node.Next != nil {
				node.Right.Next = node.Next.Left
			}
		}
	}
	return root
}
