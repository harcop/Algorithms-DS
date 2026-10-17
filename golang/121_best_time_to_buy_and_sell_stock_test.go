package leetcode

import "testing"

func TestMaxProfitExampleOne(t *testing.T) {
	got := maxProfit([]int{7, 1, 5, 3, 6, 4})
	if got != 5 {
		t.Fatalf("maxProfit() = %d, want 5", got)
	}
}

func TestMaxProfitExampleTwo(t *testing.T) {
	got := maxProfit([]int{7, 6, 4, 3, 1})
	if got != 0 {
		t.Fatalf("maxProfit() = %d, want 0", got)
	}
}
