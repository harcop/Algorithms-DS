package leetcode

// LeetCode #396 - Rotate Function
//
// Each rotation adds the total sum and drops n times the value that moved to the front.
func maxRotateFunction(nums []int) int {
	n := len(nums)
	if n == 0 {
		return 0
	}
	var sum, score int64
	for i, value := range nums {
		sum += int64(value)
		score += int64(i) * int64(value)
	}
	best := score
	for i := n - 1; i >= 1; i-- {
		score += sum - int64(n)*int64(nums[i])
		if score > best {
			best = score
		}
	}
	return int(best)
}
