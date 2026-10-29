package leetcode

// LeetCode #294 - Median of Unsorted Arrays
//
// Quickselect the middle value, or the average of the two middle values.
func findMedian(nums []int) float64 {
	n := len(nums)
	if n == 0 {
		return 0
	}
	if n%2 == 1 {
		return float64(orderStatistic(nums, n/2))
	}
	left := float64(orderStatistic(nums, n/2-1))
	right := float64(orderStatistic(nums, n/2))
	return (left + right) / 2
}

func orderStatistic(nums []int, k int) int {
	lo, hi := 0, len(nums)-1
	for lo < hi {
		pivot := orderPartition(nums, lo, hi)
		if k == pivot {
			return nums[k]
		}
		if k < pivot {
			hi = pivot - 1
		} else {
			lo = pivot + 1
		}
	}
	return nums[lo]
}

func orderPartition(nums []int, lo, hi int) int {
	mid := lo + (hi-lo)/2
	nums[mid], nums[hi] = nums[hi], nums[mid]
	pivot := nums[hi]
	i := lo
	for j := lo; j < hi; j++ {
		if nums[j] <= pivot {
			nums[i], nums[j] = nums[j], nums[i]
			i++
		}
	}
	nums[i], nums[hi] = nums[hi], nums[i]
	return i
}
