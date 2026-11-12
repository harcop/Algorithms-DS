package leetcode

import "testing"

func TestChangeExamples(t *testing.T) {
	if got := change(5, []int{1, 2, 5}); got != 4 {
		t.Fatalf("change(5) = %d, want 4", got)
	}
	if got := change(3, []int{2}); got != 0 {
		t.Fatalf("change(3) = %d, want 0", got)
	}
	if got := change(10, []int{10}); got != 1 {
		t.Fatalf("change(10) = %d, want 1", got)
	}
}
