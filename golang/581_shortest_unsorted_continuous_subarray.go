package leetcode

// LeetCode #581 - Shortest Unsorted Continuous Subarray
//
// The right edge is the last value below the running max, and the left edge is the last value above the running min.
func findUnsortedSubarray(nums []int) int {
	n := len(nums)
	left, right := -1, -1
	maxSeen, minSeen := nums[0], nums[n-1]
	for i := 1; i < n; i++ {
		if nums[i] < maxSeen {
			right = i
		} else {
			maxSeen = nums[i]
		}
		j := n - 1 - i
		if nums[j] > minSeen {
			left = j
		} else {
			minSeen = nums[j]
		}
	}
	if right == -1 {
		return 0
	}
	return right - left + 1
}
