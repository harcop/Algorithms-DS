package leetcode

import "testing"

func TestNumMatrixExample(t *testing.T) {
	matrix := NewNumMatrix([][]int{
		{3, 0, 1, 4, 2},
		{5, 6, 3, 2, 1},
		{1, 2, 0, 1, 5},
		{4, 1, 0, 1, 7},
		{1, 0, 3, 0, 5},
	})
	if got := matrix.SumRegion(2, 1, 4, 3); got != 8 {
		t.Fatalf("SumRegion(2, 1, 4, 3) = %d, want 8", got)
	}
	if got := matrix.SumRegion(1, 1, 2, 2); got != 11 {
		t.Fatalf("SumRegion(1, 1, 2, 2) = %d, want 11", got)
	}
}
