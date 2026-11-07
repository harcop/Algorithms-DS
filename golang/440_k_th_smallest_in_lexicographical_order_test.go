package leetcode

import "testing"

func TestFindKthNumberExamples(t *testing.T) {
	if got := findKthNumber(13, 2); got != 10 {
		t.Fatalf("findKthNumber(13, 2) = %d, want 10", got)
	}
	if got := findKthNumber(10, 3); got != 2 {
		t.Fatalf("findKthNumber(10, 3) = %d, want 2", got)
	}
	if got := findKthNumber(1, 1); got != 1 {
		t.Fatalf("findKthNumber(1, 1) = %d, want 1", got)
	}
}
