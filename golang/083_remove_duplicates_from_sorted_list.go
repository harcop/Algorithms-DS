package leetcode

// LeetCode #83 - Remove Duplicates from Sorted List
//
// Skip later nodes that repeat the current value, leaving one copy of each.
func deleteDuplicates(head *ListNode) *ListNode {
	cur := head
	for cur != nil && cur.Next != nil {
		if cur.Val == cur.Next.Val {
			cur.Next = cur.Next.Next
		} else {
			cur = cur.Next
		}
	}
	return head
}
