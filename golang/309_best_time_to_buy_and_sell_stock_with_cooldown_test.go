package leetcode

import "testing"

func TestMaxProfitCooldownExampleOne(t *testing.T) {
	if got := maxProfitCooldown([]int{1, 2, 3, 0, 2}); got != 3 {
		t.Fatalf("maxProfitCooldown() = %d, want 3", got)
	}
}

func TestMaxProfitCooldownExampleTwo(t *testing.T) {
	if got := maxProfitCooldown([]int{1}); got != 0 {
		t.Fatalf("maxProfitCooldown() = %d, want 0", got)
	}
}
