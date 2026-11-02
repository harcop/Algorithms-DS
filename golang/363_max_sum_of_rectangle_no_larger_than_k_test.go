package leetcode

import "testing"

func TestMaxSumSubmatrixExample(t *testing.T) {
	matrix := [][]int{{1, 0, 1}, {0, -2, 3}}
	if got := maxSumSubmatrix(matrix, 2); got != 2 {
		t.Fatalf("maxSumSubmatrix() = %d, want 2", got)
	}
}

func TestMaxSumSubmatrixWholeRow(t *testing.T) {
	matrix := [][]int{{2, 2, -1}}
	if got := maxSumSubmatrix(matrix, 3); got != 3 {
		t.Fatalf("maxSumSubmatrix() = %d, want 3", got)
	}
}
