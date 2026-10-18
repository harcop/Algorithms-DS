package leetcode

// LeetCode #141 - Linked List Cycle
//
// The fast pointer gains one step each loop, so it meets the slow pointer only inside a cycle.
func hasCycle(head *ListNode) bool {
	slow, fast := head, head
	for fast != nil && fast.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
		if slow == fast {
			return true
		}
	}
	return false
}
