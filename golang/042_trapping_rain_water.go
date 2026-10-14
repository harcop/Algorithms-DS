package leetcode

// LeetCode #42 - Trapping Rain Water
//
// Walk inward from both ends. The shorter side decides how much water the next bar can hold.
func trap(height []int) int {
	if len(height) < 3 {
		return 0
	}

	left := 0
	right := len(height) - 1
	leftMax := 0
	rightMax := 0
	water := 0

	for left < right {
		if height[left] < height[right] {
			if height[left] >= leftMax {
				leftMax = height[left]
			} else {
				water += leftMax - height[left]
			}
			left++
		} else {
			if height[right] >= rightMax {
				rightMax = height[right]
			} else {
				water += rightMax - height[right]
			}
			right--
		}
	}

	return water
}
