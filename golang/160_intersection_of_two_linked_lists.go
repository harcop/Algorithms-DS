package leetcode

// LeetCode #160 - Intersection of Two Linked Lists
//
// Skip the extra prefix of the longer list, then walk both lists until the same node appears.
func getIntersectionNode(headA, headB *ListNode) *ListNode {
	lenA, lenB := 0, 0
	for node := headA; node != nil; node = node.Next {
		lenA++
	}
	for node := headB; node != nil; node = node.Next {
		lenB++
	}
	for lenA > lenB {
		headA = headA.Next
		lenA--
	}
	for lenB > lenA {
		headB = headB.Next
		lenB--
	}
	for headA != nil && headB != nil {
		if headA == headB {
			return headA
		}
		headA = headA.Next
		headB = headB.Next
	}
	return nil
}
