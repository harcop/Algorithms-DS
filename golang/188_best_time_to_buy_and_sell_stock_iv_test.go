package leetcode

import "testing"

func TestMaxProfitIVExampleOne(t *testing.T) {
	if got := maxProfitIV(2, []int{2, 4, 1}); got != 2 {
		t.Fatalf("maxProfitIV() = %d, want 2", got)
	}
}

func TestMaxProfitIVExampleTwo(t *testing.T) {
	if got := maxProfitIV(2, []int{3, 2, 6, 5, 0, 3}); got != 7 {
		t.Fatalf("maxProfitIV() = %d, want 7", got)
	}
}
