package leetcode

// LeetCode #31 - Next Permutation
//
// Find the longest non-increasing suffix, swap its predecessor with the next larger
// suffix value, then reverse the suffix into increasing order.
func nextPermutation(nums []int) {
	n := len(nums)
	if n < 2 {
		return
	}

	i := n - 1
	for i > 0 && nums[i-1] >= nums[i] {
		i--
	}

	if i > 0 {
		pivot := i - 1
		j := n - 1
		for nums[j] <= nums[pivot] {
			j--
		}
		nums[pivot], nums[j] = nums[j], nums[pivot]
	}

	for left, right := i, n-1; left < right; left, right = left+1, right-1 {
		nums[left], nums[right] = nums[right], nums[left]
	}
}
