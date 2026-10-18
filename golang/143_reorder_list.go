package leetcode

// LeetCode #143 - Reorder List
//
// Reverse the second half, then weave it into the first half.
func reorderList(head *ListNode) {
	if head == nil || head.Next == nil {
		return
	}

	slow, fast := head, head
	for fast.Next != nil && fast.Next.Next != nil {
		slow = slow.Next
		fast = fast.Next.Next
	}
	second := reverseHalf(slow.Next)
	slow.Next = nil

	first := head
	for second != nil {
		nextFirst, nextSecond := first.Next, second.Next
		first.Next = second
		second.Next = nextFirst
		first = nextFirst
		second = nextSecond
	}
}

func reverseHalf(head *ListNode) *ListNode {
	var prev *ListNode
	for head != nil {
		next := head.Next
		head.Next = prev
		prev = head
		head = next
	}
	return prev
}
