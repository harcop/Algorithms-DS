package leetcode

// LeetCode #493 - Reverse Pairs
//
// Merge sort counts, in each half, how many right-side values are less than half of a left-side value.
func reversePairs(nums []int) int {
	temp := make([]int, len(nums))
	return int(countReversePairs(nums, temp, 0, len(nums)))
}

func countReversePairs(nums, temp []int, left, right int) int64 {
	if right-left <= 1 {
		return 0
	}
	mid := left + (right-left)/2
	count := countReversePairs(nums, temp, left, mid) + countReversePairs(nums, temp, mid, right)
	j := mid
	for i := left; i < mid; i++ {
		for j < right && int64(nums[i]) > 2*int64(nums[j]) {
			j++
		}
		count += int64(j - mid)
	}
	i, k, rightIndex := left, left, mid
	for i < mid && rightIndex < right {
		if nums[i] <= nums[rightIndex] {
			temp[k] = nums[i]
			i++
		} else {
			temp[k] = nums[rightIndex]
			rightIndex++
		}
		k++
	}
	for i < mid {
		temp[k] = nums[i]
		i++
		k++
	}
	for rightIndex < right {
		temp[k] = nums[rightIndex]
		rightIndex++
		k++
	}
	copy(nums[left:right], temp[left:right])
	return count
}
