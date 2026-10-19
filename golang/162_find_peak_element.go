package leetcode

// LeetCode #162 - Find Peak Element
//
// A peak is larger than its neighbors, so follow the rising side of the array.
func findPeakElement(nums []int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		mid := lo + (hi-lo)/2
		if nums[mid] < nums[mid+1] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return lo
}
