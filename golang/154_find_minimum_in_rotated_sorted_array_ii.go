package leetcode

// LeetCode #154 - Find Minimum in Rotated Sorted Array II
//
// LeetCode names this findMin; that name already belongs to the distinct-value version (#153).
// Duplicates hide which half is sorted, so shrink the right bound by one when the ends match.
func findMinII(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := lo + (hi-lo)/2
		if nums[mid] > nums[hi] {
			lo = mid + 1
		} else if nums[mid] < nums[hi] {
			hi = mid
		} else {
			hi--
		}
	}
	return nums[lo]
}
