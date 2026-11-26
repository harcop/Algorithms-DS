package leetcode

// LeetCode #699 - Falling Squares
//
// A square lands on the tallest square whose interval overlaps it, and the running maximum is recorded.
func fallingSquares(positions [][]int) []int {
	heights := make([]int, len(positions))
	answer := make([]int, len(positions))
	best := 0
	for i, position := range positions {
		left, side := position[0], position[1]
		right := left + side
		height := side
		for j := 0; j < i; j++ {
			otherLeft := positions[j][0]
			otherRight := otherLeft + positions[j][1]
			if left < otherRight && otherLeft < right {
				height = max(height, heights[j]+side)
			}
		}
		heights[i] = height
		best = max(best, height)
		answer[i] = best
	}
	return answer
}
