package leetcode

// LeetCode #153 - Find Minimum in Rotated Sorted Array
//
// The minimum is in the half that is out of order with the right end.
func findMin(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := lo + (hi-lo)/2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return nums[lo]
}
