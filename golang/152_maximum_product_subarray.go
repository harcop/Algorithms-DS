package leetcode

// LeetCode #152 - Maximum Product Subarray
//
// Keep the largest and smallest products ending here, because a negative can flip them.
func maxProduct(nums []int) int {
	maxEnding, minEnding, best := nums[0], nums[0], nums[0]
	for _, value := range nums[1:] {
		if value < 0 {
			maxEnding, minEnding = minEnding, maxEnding
		}
		if value > maxEnding*value {
			maxEnding = value
		} else {
			maxEnding *= value
		}
		if value < minEnding*value {
			minEnding = value
		} else {
			minEnding *= value
		}
		if maxEnding > best {
			best = maxEnding
		}
	}
	return best
}
