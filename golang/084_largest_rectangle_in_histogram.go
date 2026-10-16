package leetcode

// LeetCode #84 - Largest Rectangle in Histogram
//
// Pop bars taller than the current one and measure the widest rectangle each popped bar can form.
func largestRectangleArea(heights []int) int {
	stack := make([]int, 0, len(heights))
	maxArea := 0

	for i := 0; i <= len(heights); i++ {
		curH := 0
		if i < len(heights) {
			curH = heights[i]
		}
		for len(stack) > 0 && heights[stack[len(stack)-1]] > curH {
			idx := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			width := i
			if len(stack) > 0 {
				width = i - stack[len(stack)-1] - 1
			}
			if area := heights[idx] * width; area > maxArea {
				maxArea = area
			}
		}
		if i < len(heights) {
			stack = append(stack, i)
		}
	}
	return maxArea
}
