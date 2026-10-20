package leetcode

// LeetCode #173 - Binary Search Tree Iterator
//
// Keep the left spine on a stack so the next call yields the next inorder value.
type BSTIterator struct {
	stack []*TreeNode
}

func NewBSTIterator(root *TreeNode) BSTIterator {
	it := BSTIterator{}
	it.pushLeft(root)
	return it
}

func (it *BSTIterator) pushLeft(node *TreeNode) {
	for node != nil {
		it.stack = append(it.stack, node)
		node = node.Left
	}
}

func (it *BSTIterator) Next() int {
	n := len(it.stack)
	node := it.stack[n-1]
	it.stack = it.stack[:n-1]
	it.pushLeft(node.Right)
	return node.Val
}

func (it *BSTIterator) HasNext() bool {
	return len(it.stack) > 0
}
