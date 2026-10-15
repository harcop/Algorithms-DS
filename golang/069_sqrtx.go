package leetcode

// LeetCode #69 - Sqrt(x)
//
// Binary search the largest integer whose square does not exceed x.
func mySqrt(x int) int {
	if x < 2 {
		return x
	}

	target := int64(x)
	left, right := int64(1), target
	for left <= right {
		mid := left + (right-left)/2
		square := mid * mid
		if square == target {
			return int(mid)
		}
		if square < target {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return int(right)
}
