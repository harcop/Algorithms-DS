package leetcode

// LeetCode #148 - Sort List
//
// Split the list in half, sort each half, then merge the sorted halves.
func sortList(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}
	slow, fast := head, head.Next
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	mid := slow.Next
	slow.Next = nil
	return mergeTwoLists(sortList(head), sortList(mid))
}
