package leetcode

// LeetCode #391 - Perfect Rectangle
//
// The rectangles cover one rectangle when their area matches the bounding box and only its four corners stay unpaired.
func isRectangleCover(rectangles [][]int) bool {
	if len(rectangles) == 0 {
		return false
	}
	var area int64
	minX, minY := rectangles[0][0], rectangles[0][1]
	maxX, maxY := rectangles[0][2], rectangles[0][3]
	corners := map[[2]int]int{}
	for _, rect := range rectangles {
		x1, y1, x2, y2 := rect[0], rect[1], rect[2], rect[3]
		if x1 < minX {
			minX = x1
		}
		if y1 < minY {
			minY = y1
		}
		if x2 > maxX {
			maxX = x2
		}
		if y2 > maxY {
			maxY = y2
		}
		area += int64(x2-x1) * int64(y2-y1)
		for _, corner := range [][2]int{{x1, y1}, {x1, y2}, {x2, y1}, {x2, y2}} {
			corners[corner]++
		}
	}
	if area != int64(maxX-minX)*int64(maxY-minY) {
		return false
	}
	expected := map[[2]int]struct{}{
		{minX, minY}: {},
		{minX, maxY}: {},
		{maxX, minY}: {},
		{maxX, maxY}: {},
	}
	for corner, count := range corners {
		if count%2 == 0 {
			continue
		}
		if count != 1 {
			return false
		}
		if _, ok := expected[corner]; !ok {
			return false
		}
		delete(expected, corner)
	}
	return len(expected) == 0
}
