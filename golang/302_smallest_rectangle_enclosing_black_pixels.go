package leetcode

// LeetCode #302 - Smallest Rectangle Enclosing Black Pixels
//
// The black pixels set the top, bottom, left, and right edges of the rectangle.
func minArea(image [][]byte, x int, y int) int {
	top, bottom, left, right := x, x, y, y
	for i := range image {
		for j := range image[i] {
			if image[i][j] != '1' {
				continue
			}
			if i < top {
				top = i
			}
			if i > bottom {
				bottom = i
			}
			if j < left {
				left = j
			}
			if j > right {
				right = j
			}
		}
	}
	return (bottom - top + 1) * (right - left + 1)
}
