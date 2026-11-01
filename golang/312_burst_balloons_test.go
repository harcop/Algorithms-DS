package leetcode

import "testing"

func TestMaxCoinsExampleOne(t *testing.T) {
	if got := maxCoins([]int{3, 1, 5, 8}); got != 167 {
		t.Fatalf("maxCoins() = %d, want 167", got)
	}
}
