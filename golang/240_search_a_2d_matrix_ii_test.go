package leetcode

import "testing"

func matrixIIExample() [][]int {
	return [][]int{
		{1, 4, 7, 11, 15},
		{2, 5, 8, 12, 19},
		{3, 6, 9, 16, 22},
		{10, 13, 14, 17, 24},
		{18, 21, 23, 26, 30},
	}
}

func TestSearchMatrixIIExampleOne(t *testing.T) {
	if !searchMatrixII(matrixIIExample(), 5) {
		t.Fatal("searchMatrixII() = false, want true")
	}
}

func TestSearchMatrixIIExampleTwo(t *testing.T) {
	if searchMatrixII(matrixIIExample(), 20) {
		t.Fatal("searchMatrixII() = true, want false")
	}
}
