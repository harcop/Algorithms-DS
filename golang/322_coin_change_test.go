package leetcode

import "testing"

func TestCoinChangeExampleOne(t *testing.T) {
	if got := coinChange([]int{1, 2, 5}, 11); got != 3 {
		t.Fatalf("coinChange() = %d, want 3", got)
	}
}

func TestCoinChangeExampleTwo(t *testing.T) {
	if got := coinChange([]int{2}, 3); got != -1 {
		t.Fatalf("coinChange() = %d, want -1", got)
	}
}
