package leetcode

// LeetCode #24 - Swap Nodes in Pairs
//
// Rewire each pair so the second node comes before the first, then advance by two.
func swapPairs(head *ListNode) *ListNode {
	dummy := &ListNode{Next: head}
	prev := dummy

	for prev.Next != nil && prev.Next.Next != nil {
		first := prev.Next
		second := first.Next
		first.Next = second.Next
		second.Next = first
		prev.Next = second
		prev = first
	}

	return dummy.Next
}
