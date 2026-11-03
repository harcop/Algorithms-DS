package leetcode

import "testing"

func TestKthSmallestMatrixExample(t *testing.T) {
	matrix := [][]int{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}
	if got := kthSmallestMatrix(matrix, 8); got != 13 {
		t.Fatalf("kthSmallestMatrix() = %d, want 13", got)
	}
	if got := kthSmallestMatrix(matrix, 1); got != 1 {
		t.Fatalf("kthSmallestMatrix() = %d, want 1", got)
	}
}
