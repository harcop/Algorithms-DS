package leetcode

import "testing"

func TestRotateExampleOne(t *testing.T) {
	matrix := [][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}}
	rotate(matrix)
	want := [][]int{{7, 4, 1}, {8, 5, 2}, {9, 6, 3}}
	if !sameMatrix(matrix, want) {
		t.Fatalf("rotate() = %v, want %v", matrix, want)
	}
}

func TestRotateExampleTwo(t *testing.T) {
	matrix := [][]int{
		{5, 1, 9, 11},
		{2, 4, 8, 10},
		{13, 3, 6, 7},
		{15, 14, 12, 16},
	}
	rotate(matrix)
	want := [][]int{
		{15, 13, 2, 5},
		{14, 3, 4, 1},
		{12, 6, 8, 9},
		{16, 7, 10, 11},
	}
	if !sameMatrix(matrix, want) {
		t.Fatalf("rotate() = %v, want %v", matrix, want)
	}
}

func sameMatrix(a, b [][]int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if !sameInts(a[i], b[i]) {
			return false
		}
	}
	return true
}
