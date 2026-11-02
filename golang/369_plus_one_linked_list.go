package leetcode

// LeetCode #369 - Plus One Linked List
//
// Add one at the least significant digit and grow a new head when the carry remains.
func plusOneLinkedList(head *ListNode) *ListNode {
	var digits []int
	for node := head; node != nil; node = node.Next {
		digits = append(digits, node.Val)
	}
	carry := 1
	for i := len(digits) - 1; i >= 0 && carry > 0; i-- {
		sum := digits[i] + carry
		digits[i] = sum % 10
		carry = sum / 10
	}
	if carry > 0 {
		digits = append([]int{carry}, digits...)
	}
	return vecToList(digits)
}
