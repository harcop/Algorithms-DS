package leetcode

// LeetCode #540 - Single Element in a Sorted Array
//
// Pairs occupy even-odd indexes until the single element, so the search follows the side where that pattern breaks.
func singleNonDuplicate(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := lo + (hi-lo)/2
		if mid%2 == 1 {
			if nums[mid] == nums[mid-1] {
				lo = mid + 1
				continue
			}
			hi = mid
			continue
		}
		if mid+1 < len(nums) && nums[mid] == nums[mid+1] {
			lo = mid + 2
			continue
		}
		hi = mid
	}
	return nums[lo]
}
