package leetcode

// LeetCode #234 - Palindrome Linked List
//
// LeetCode names this isPalindrome; that name already belongs to the integer version (#9).
// Copy the values and compare them from both ends.
func isPalindromeList(head *ListNode) bool {
	var values []int
	for head != nil {
		values = append(values, head.Val)
		head = head.Next
	}
	for i, j := 0, len(values)-1; i < j; i, j = i+1, j-1 {
		if values[i] != values[j] {
			return false
		}
	}
	return true
}
