package leetcode

// LeetCode #237 - Delete Node in a Linked List
//
// Copy the next node into this one, then skip the next node.
func deleteNode(node *ListNode) {
	node.Val = node.Next.Val
	node.Next = node.Next.Next
}
