package leetcode

// LeetCode #25 - Reverse Nodes in k-Group
//
// Reverse each complete group of k nodes in place and leave a trailing partial group unchanged.
func reverseKGroup(head *ListNode, k int) *ListNode {
	if k <= 1 {
		return head
	}

	dummy := &ListNode{Next: head}
	groupPrev := dummy

	for {
		kth := groupPrev
		for i := 0; i < k; i++ {
			kth = kth.Next
			if kth == nil {
				return dummy.Next
			}
		}

		groupNext := kth.Next
		prev := groupNext
		curr := groupPrev.Next
		for curr != groupNext {
			next := curr.Next
			curr.Next = prev
			prev = curr
			curr = next
		}

		newGroupPrev := groupPrev.Next
		groupPrev.Next = kth
		groupPrev = newGroupPrev
	}
}
