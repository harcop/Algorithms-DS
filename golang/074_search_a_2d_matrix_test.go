package leetcode

import "testing"

func TestSearchMatrixExampleOne(t *testing.T) {
	matrix := [][]int{{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}}
	if !searchMatrix(matrix, 3) {
		t.Fatal("searchMatrix() = false, want true")
	}
}

func TestSearchMatrixExampleTwo(t *testing.T) {
	matrix := [][]int{{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}}
	if searchMatrix(matrix, 13) {
		t.Fatal("searchMatrix() = true, want false")
	}
}
