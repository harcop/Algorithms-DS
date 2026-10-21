package leetcode

// LeetCode #209 - Minimum Size Subarray Sum
//
// Grow a window until it reaches the target, then shrink it from the left.
func minSubArrayLen(target int, nums []int) int {
	lo, sum, best := 0, 0, len(nums)+1
	for hi, value := range nums {
		sum += value
		for sum >= target {
			if span := hi - lo + 1; span < best {
				best = span
			}
			sum -= nums[lo]
			lo++
		}
	}
	if best > len(nums) {
		return 0
	}
	return best
}
