package leetcode

// LeetCode #109 - Convert Sorted List to Binary Search Tree
//
// Copy the sorted list into an array, then build the balanced tree from the middle outward.
func sortedListToBST(head *ListNode) *TreeNode {
	var nums []int
	for head != nil {
		nums = append(nums, head.Val)
		head = head.Next
	}
	return sortedArrayToBST(nums)
}
