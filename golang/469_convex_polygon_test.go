package leetcode

import "testing"

func TestIsConvexExamples(t *testing.T) {
	square := [][]int{{0, 0}, {0, 1}, {1, 1}, {1, 0}}
	if !isConvex(square) {
		t.Fatal("isConvex(square) = false, want true")
	}
	dented := [][]int{{0, 0}, {0, 10}, {10, 10}, {10, 0}, {5, 5}}
	if isConvex(dented) {
		t.Fatal("isConvex(dented) = true, want false")
	}
}
