package leetcode

import "testing"

func TestRandomRectExamples(t *testing.T) {
	rects := [][]int{{-2, -2, 1, 1}, {2, 2, 4, 6}}
	picker := NewRandomRect(rects)
	for i := 0; i < 50; i++ {
		point := picker.Pick()
		if !pointInRects(point, rects) {
			t.Fatalf("Pick() = %v, outside the rectangles", point)
		}
	}
	single := [][]int{{1, 1, 5, 5}}
	picker = NewRandomRect(single)
	for i := 0; i < 20; i++ {
		point := picker.Pick()
		if !pointInRects(point, single) {
			t.Fatalf("Pick() = %v, outside the rectangle", point)
		}
	}
}

func pointInRects(point []int, rects [][]int) bool {
	for _, rect := range rects {
		if point[0] >= rect[0] && point[0] <= rect[2] && point[1] >= rect[1] && point[1] <= rect[3] {
			return true
		}
	}
	return false
}
