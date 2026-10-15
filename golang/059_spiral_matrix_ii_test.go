package leetcode

import "testing"

func TestGenerateMatrixExampleOne(t *testing.T) {
	got := generateMatrix(3)
	want := [][]int{{1, 2, 3}, {8, 9, 4}, {7, 6, 5}}
	if !sameMatrix(got, want) {
		t.Fatalf("generateMatrix() = %v, want %v", got, want)
	}
}

func TestGenerateMatrixExampleTwo(t *testing.T) {
	got := generateMatrix(1)
	want := [][]int{{1}}
	if !sameMatrix(got, want) {
		t.Fatalf("generateMatrix() = %v, want %v", got, want)
	}
}
