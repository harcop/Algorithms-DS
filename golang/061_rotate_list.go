package leetcode

// LeetCode #61 - Rotate List
//
// Close the list into a ring, then break it just before the new head.
func rotateRight(head *ListNode, k int) *ListNode {
	if head == nil || head.Next == nil || k == 0 {
		return head
	}

	n := 1
	tail := head
	for tail.Next != nil {
		tail = tail.Next
		n++
	}
	k %= n
	if k == 0 {
		return head
	}

	newTail := head
	for i := 0; i < n-k-1; i++ {
		newTail = newTail.Next
	}
	newHead := newTail.Next
	newTail.Next = nil
	tail.Next = head
	return newHead
}
