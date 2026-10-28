package leetcode

// LeetCode #287 - Find the Duplicate Number
//
// Treat the values as links and use Floyd's cycle finding to land on the repeated value.
func findDuplicate(nums []int) int {
	slow, fast := nums[0], nums[0]
	for {
		slow = nums[slow]
		fast = nums[nums[fast]]
		if slow == fast {
			break
		}
	}
	fast = nums[0]
	for slow != fast {
		slow = nums[slow]
		fast = nums[fast]
	}
	return slow
}
