package leetcode

// LeetCode #53 - Maximum Subarray
//
// Extend the current sum when it helps, otherwise start a new subarray at this value.
func maxSubArray(nums []int) int {
	best := nums[0]
	current := nums[0]
	for _, num := range nums[1:] {
		if current+num > num {
			current += num
		} else {
			current = num
		}
		if current > best {
			best = current
		}
	}
	return best
}
