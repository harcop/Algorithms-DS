package leetcode

// LeetCode #328 - Odd Even Linked List
//
// Keep the odd-position nodes in place and append the even-position chain after them.
func oddEvenList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	odd := head
	evenHead := head.Next
	even := evenHead
	for even != nil && even.Next != nil {
		odd.Next = even.Next
		odd = odd.Next
		even.Next = odd.Next
		even = even.Next
	}
	odd.Next = evenHead
	return head
}
