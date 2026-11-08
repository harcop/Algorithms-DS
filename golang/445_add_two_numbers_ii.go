package leetcode

// LeetCode #445 - Add Two Numbers II
//
// LeetCode names this addTwoNumbers; that name already belongs to the reversed-digit lists (#2).
// Push both lists onto stacks so the ones place is added first, then prepend each digit of the sum.
func addTwoNumbersII(l1 *ListNode, l2 *ListNode) *ListNode {
	var a, b []int
	for l1 != nil {
		a = append(a, l1.Val)
		l1 = l1.Next
	}
	for l2 != nil {
		b = append(b, l2.Val)
		l2 = l2.Next
	}
	carry := 0
	var head *ListNode
	for len(a) > 0 || len(b) > 0 || carry > 0 {
		sum := carry
		if len(a) > 0 {
			sum += a[len(a)-1]
			a = a[:len(a)-1]
		}
		if len(b) > 0 {
			sum += b[len(b)-1]
			b = b[:len(b)-1]
		}
		carry = sum / 10
		head = &ListNode{Val: sum % 10, Next: head}
	}
	return head
}
