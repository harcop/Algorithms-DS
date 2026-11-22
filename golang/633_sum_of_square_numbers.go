package leetcode

import "math"

// LeetCode #633 - Sum of Square Numbers
//
// Two roots move toward each other until their squares add up to c.
func judgeSquareSum(c int) bool {
	low := int64(0)
	high := int64(math.Sqrt(float64(c)))
	target := int64(c)
	for low <= high {
		sum := low*low + high*high
		if sum == target {
			return true
		}
		if sum < target {
			low++
		} else {
			high--
		}
	}
	return false
}
