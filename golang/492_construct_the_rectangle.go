package leetcode

import "math"

// LeetCode #492 - Construct the Rectangle
//
// The width is the largest divisor that does not exceed the square root, and the length is the other factor.
func constructRectangle(area int) []int {
	width := int(math.Sqrt(float64(area)))
	for int64(width+1)*int64(width+1) <= int64(area) {
		width++
	}
	for area%width != 0 {
		width--
	}
	return []int{area / width, width}
}
