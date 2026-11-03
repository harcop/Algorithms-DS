package leetcode

// LeetCode #376 - Wiggle Subsequence
//
// Track the longest wiggle that ends on a rise and the one that ends on a fall.
func wiggleMaxLength(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	up, down := 1, 1
	for i := 1; i < len(nums); i++ {
		if nums[i] > nums[i-1] {
			up = down + 1
		} else if nums[i] < nums[i-1] {
			down = up + 1
		}
	}
	if up > down {
		return up
	}
	return down
}
