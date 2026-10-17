package leetcode

import "testing"

func TestMaxProfitIIIExampleOne(t *testing.T) {
	got := maxProfitIII([]int{3, 3, 5, 0, 0, 3, 1, 4})
	if got != 6 {
		t.Fatalf("maxProfitIII() = %d, want 6", got)
	}
}

func TestMaxProfitIIIExampleTwo(t *testing.T) {
	got := maxProfitIII([]int{1, 2, 3, 4, 5})
	if got != 4 {
		t.Fatalf("maxProfitIII() = %d, want 4", got)
	}
}
