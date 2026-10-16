package leetcode

// LeetCode #92 - Reverse Linked List II
//
// Walk to the node before left, then move each following node to the front of the range.
func reverseBetween(head *ListNode, left int, right int) *ListNode {
	if head == nil || left == right {
		return head
	}

	dummy := &ListNode{Next: head}
	prev := dummy
	for i := 1; i < left; i++ {
		prev = prev.Next
	}

	curr := prev.Next
	for i := 0; i < right-left; i++ {
		next := curr.Next
		curr.Next = next.Next
		next.Next = prev.Next
		prev.Next = next
	}
	return dummy.Next
}
