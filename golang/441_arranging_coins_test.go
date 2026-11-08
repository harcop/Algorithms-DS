package leetcode

import "testing"

func TestArrangeCoinsExamples(t *testing.T) {
	if got := arrangeCoins(5); got != 2 {
		t.Fatalf("arrangeCoins(5) = %d, want 2", got)
	}
	if got := arrangeCoins(8); got != 3 {
		t.Fatalf("arrangeCoins(8) = %d, want 3", got)
	}
}
