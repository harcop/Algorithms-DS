package leetcode

import "testing"

func TestSetZeroesExampleOne(t *testing.T) {
	matrix := [][]int{{1, 1, 1}, {1, 0, 1}, {1, 1, 1}}
	setZeroes(matrix)
	want := [][]int{{1, 0, 1}, {0, 0, 0}, {1, 0, 1}}
	if !sameMatrix(matrix, want) {
		t.Fatalf("setZeroes() = %v, want %v", matrix, want)
	}
}

func TestSetZeroesExampleTwo(t *testing.T) {
	matrix := [][]int{{0, 1, 2, 0}, {3, 4, 5, 2}, {1, 3, 1, 5}}
	setZeroes(matrix)
	want := [][]int{{0, 0, 0, 0}, {0, 4, 5, 0}, {0, 3, 1, 0}}
	if !sameMatrix(matrix, want) {
		t.Fatalf("setZeroes() = %v, want %v", matrix, want)
	}
}
