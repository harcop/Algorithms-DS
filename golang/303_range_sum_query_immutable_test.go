package leetcode

import "testing"

func TestNumArrayExample(t *testing.T) {
	array := NewNumArray([]int{-2, 0, 3, -5, 2, -1})
	if got := array.SumRange(0, 2); got != 1 {
		t.Fatalf("SumRange(0, 2) = %d, want 1", got)
	}
	if got := array.SumRange(2, 5); got != -1 {
		t.Fatalf("SumRange(2, 5) = %d, want -1", got)
	}
	if got := array.SumRange(0, 5); got != -3 {
		t.Fatalf("SumRange(0, 5) = %d, want -3", got)
	}
}
