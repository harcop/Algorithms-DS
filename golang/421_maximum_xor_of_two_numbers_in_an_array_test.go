package leetcode

import "testing"

func TestFindMaximumXORExamples(t *testing.T) {
	if got := findMaximumXOR([]int{3, 10, 5, 25, 2, 8}); got != 28 {
		t.Fatalf("findMaximumXOR() = %d, want 28", got)
	}
	if got := findMaximumXOR([]int{8, 10, 2}); got != 10 {
		t.Fatalf("findMaximumXOR() = %d, want 10", got)
	}
}
