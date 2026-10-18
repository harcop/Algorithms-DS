package leetcode

// LeetCode #142 - Linked List Cycle II
//
// After the pointers meet, walking one from the head and one from the meeting point finds the cycle start.
func detectCycle(head *ListNode) *ListNode {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
		if slow != fast {
			continue
		}
		slow = head
		for slow != fast {
			slow = slow.Next
			fast = fast.Next
		}
		return slow
	}
	return nil
}
