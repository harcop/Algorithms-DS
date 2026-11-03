package leetcode

import "testing"

func TestSuperPowExamples(t *testing.T) {
	if got := superPow(2, []int{3}); got != 8 {
		t.Fatalf("superPow() = %d, want 8", got)
	}
	if got := superPow(2, []int{1, 0}); got != 1024 {
		t.Fatalf("superPow() = %d, want 1024", got)
	}
	if got := superPow(1, []int{4, 3, 3, 8, 5, 2}); got != 1 {
		t.Fatalf("superPow() = %d, want 1", got)
	}
}
