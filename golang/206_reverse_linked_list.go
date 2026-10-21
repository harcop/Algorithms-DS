package leetcode

// LeetCode #206 - Reverse Linked List
//
// Point each node at the one before it while walking forward.
func reverseList(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		next := head.Next
		head.Next = prev
		prev = head
		head = next
	}
	return prev
}
