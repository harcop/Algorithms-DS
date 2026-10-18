package leetcode

// LeetCode #147 - Insertion Sort List
//
// Pull each node out and insert it into a growing sorted list.
func insertionSortList(head *ListNode) *ListNode {
	dummy := &ListNode{}
	for head != nil {
		next := head.Next
		prev := dummy
		for prev.Next != nil && prev.Next.Val < head.Val {
			prev = prev.Next
		}
		head.Next = prev.Next
		prev.Next = head
		head = next
	}
	return dummy.Next
}
