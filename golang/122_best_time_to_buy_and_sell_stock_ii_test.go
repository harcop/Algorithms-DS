package leetcode

import "testing"

func TestMaxProfitIIExampleOne(t *testing.T) {
	got := maxProfitII([]int{7, 1, 5, 3, 6, 4})
	if got != 7 {
		t.Fatalf("maxProfitII() = %d, want 7", got)
	}
}

func TestMaxProfitIIExampleTwo(t *testing.T) {
	got := maxProfitII([]int{1, 2, 3, 4, 5})
	if got != 4 {
		t.Fatalf("maxProfitII() = %d, want 4", got)
	}
}
