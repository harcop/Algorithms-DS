package leetcode

// LeetCode #223 - Rectangle Area
//
// Add the two areas and subtract the overlap, which is empty when the rectangles miss.
func computeArea(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2 int) int {
	a := int64(ax2-ax1) * int64(ay2-ay1)
	b := int64(bx2-bx1) * int64(by2-by1)
	width := int64(min(ax2, bx2) - max(ax1, bx1))
	height := int64(min(ay2, by2) - max(ay1, by1))
	overlap := int64(0)
	if width > 0 && height > 0 {
		overlap = width * height
	}
	return int(a + b - overlap)
}
