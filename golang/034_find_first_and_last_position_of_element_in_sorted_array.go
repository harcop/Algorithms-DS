package leetcode

// LeetCode #34 - Find First and Last Position of Element in Sorted Array
//
// Two lower-bound searches locate the first index of target and the first index after it.
func searchRange(nums []int, target int) []int {
	left := lowerBound(nums, target)
	if left == len(nums) || nums[left] != target {
		return []int{-1, -1}
	}
	right := lowerBound(nums, target+1) - 1
	return []int{left, right}
}

func lowerBound(nums []int, target int) int {
	left := 0
	right := len(nums)
	for left < right {
		mid := left + (right-left)/2
		if nums[mid] < target {
			left = mid + 1
		} else {
			right = mid
		}
	}
	return left
}
