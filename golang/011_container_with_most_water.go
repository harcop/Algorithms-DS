package leetcode

// LeetCode #11 - Container With Most Water
//
// Two pointers move inward from both ends, always advancing the shorter line.
func maxArea(height []int) int {
	if len(height) < 2 {
		return 0
	}

	left := 0
	right := len(height) - 1
	best := 0

	for left < right {
		h := height[left]
		if height[right] < h {
			h = height[right]
		}
		area := h * (right - left)
		if area > best {
			best = area
		}

		if height[left] < height[right] {
			left++
		} else {
			right--
		}
	}

	return best
}
