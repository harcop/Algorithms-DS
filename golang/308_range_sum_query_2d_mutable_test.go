package leetcode

import "testing"

func TestNumMatrixIIExample(t *testing.T) {
	matrix := NewNumMatrixII([][]int{
		{3, 0, 1, 4, 2},
		{5, 6, 3, 2, 1},
		{1, 2, 0, 1, 5},
		{4, 1, 0, 1, 7},
		{1, 0, 3, 0, 5},
	})
	if got := matrix.SumRegion(2, 1, 4, 3); got != 8 {
		t.Fatalf("SumRegion(2, 1, 4, 3) = %d, want 8", got)
	}
	matrix.Update(3, 2, 2)
	if got := matrix.SumRegion(2, 1, 4, 3); got != 10 {
		t.Fatalf("SumRegion(2, 1, 4, 3) = %d, want 10", got)
	}
}
