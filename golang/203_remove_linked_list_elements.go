package leetcode

// LeetCode #203 - Remove Linked List Elements
//
// Rebuild the list, skipping every node whose value matches.
func removeElements(head *ListNode, val int) *ListNode {
	dummy := &ListNode{}
	tail := dummy
	for head != nil {
		next := head.Next
		if head.Val != val {
			head.Next = nil
			tail.Next = head
			tail = head
		}
		head = next
	}
	return dummy.Next
}
