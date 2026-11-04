package leetcode

import "testing"

func TestIsRectangleCoverExamples(t *testing.T) {
	covered := [][]int{{1, 1, 3, 3}, {3, 1, 4, 2}, {3, 2, 4, 3}, {1, 3, 2, 4}, {2, 3, 4, 4}}
	if !isRectangleCover(covered) {
		t.Fatal("isRectangleCover() = false, want true")
	}
	if !isRectangleCover([][]int{{0, 0, 1, 1}}) {
		t.Fatal("isRectangleCover() = false, want true")
	}
	if isRectangleCover([][]int{{0, 0, 1, 1}, {1, 0, 2, 2}}) {
		t.Fatal("isRectangleCover() = true, want false")
	}
}
