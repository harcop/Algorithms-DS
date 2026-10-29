package leetcode

import "testing"

func TestNumArrayIIExample(t *testing.T) {
	array := NewNumArrayII([]int{1, 3, 5})
	if got := array.SumRange(0, 2); got != 9 {
		t.Fatalf("SumRange(0, 2) = %d, want 9", got)
	}
	array.Update(1, 2)
	if got := array.SumRange(0, 2); got != 8 {
		t.Fatalf("SumRange(0, 2) = %d, want 8", got)
	}
}
