package leetcode

// MultiNode is a doubly linked list node that may also point at a child list.
type MultiNode struct {
	Val   int
	Prev  *MultiNode
	Next  *MultiNode
	Child *MultiNode
}

// LeetCode #430 - Flatten a Multilevel Doubly Linked List
//
// A child list is spliced between a node and its next sibling, then the child pointer is cleared.
func flattenMultilevel(head *MultiNode) *MultiNode {
	var walk func(*MultiNode) *MultiNode
	walk = func(node *MultiNode) *MultiNode {
		var tail *MultiNode
		for node != nil {
			next := node.Next
			if node.Child != nil {
				childTail := walk(node.Child)
				node.Next = node.Child
				node.Child.Prev = node
				node.Child = nil
				if next != nil {
					childTail.Next = next
					next.Prev = childTail
				}
				tail = childTail
				node = next
				continue
			}
			tail = node
			node = next
		}
		return tail
	}
	walk(head)
	return head
}
