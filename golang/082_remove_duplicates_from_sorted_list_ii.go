package leetcode

// LeetCode #82 - Remove Duplicates from Sorted List II
//
// LeetCode names this deleteDuplicates; #83 uses that name for the version that keeps one copy.
// Drop every value that appears more than once, and keep values that appear once.
func deleteDuplicatesII(head *ListNode) *ListNode {
	dummy := &ListNode{Next: head}
	prev := dummy
	for head != nil {
		if head.Next != nil && head.Val == head.Next.Val {
			val := head.Val
			for head != nil && head.Val == val {
				head = head.Next
			}
			prev.Next = head
			continue
		}
		prev = head
		head = head.Next
	}
	return dummy.Next
}
