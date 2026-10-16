package leetcode

// LeetCode #86 - Partition List
//
// Splice nodes less than x onto one list and the rest onto another, then join them.
func partition(head *ListNode, x int) *ListNode {
	before := &ListNode{}
	after := &ListNode{}
	less, greater := before, after

	for head != nil {
		next := head.Next
		head.Next = nil
		if head.Val < x {
			less.Next = head
			less = head
		} else {
			greater.Next = head
			greater = head
		}
		head = next
	}
	less.Next = after.Next
	return before.Next
}
