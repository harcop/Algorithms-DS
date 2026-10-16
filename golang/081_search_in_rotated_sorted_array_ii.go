package leetcode

// LeetCode #81 - Search in Rotated Sorted Array II
//
// LeetCode names this search; that name already belongs to the distinct-value version (#33).
// Binary search either half, and skip a step when duplicates hide which half is sorted.
func searchII(nums []int, target int) bool {
	lo, hi := 0, len(nums)
	for lo < hi {
		mid := lo + (hi-lo)/2
		if nums[mid] == target {
			return true
		}
		last := hi - 1
		if nums[lo] == nums[mid] && nums[mid] == nums[last] {
			lo++
			continue
		}
		if nums[lo] <= nums[mid] {
			if nums[lo] <= target && target < nums[mid] {
				hi = mid
			} else {
				lo = mid + 1
			}
		} else if nums[mid] < target && target <= nums[last] {
			lo = mid + 1
		} else {
			hi = mid
		}
	}
	return false
}
