package leetcode

import "testing"

func TestRemoveBoxesExamples(t *testing.T) {
	if got := removeBoxes([]int{1, 3, 2, 2, 2, 3, 4, 3, 1}); got != 23 {
		t.Fatalf("removeBoxes = %d, want 23", got)
	}
	if got := removeBoxes([]int{1, 1, 1}); got != 9 {
		t.Fatalf("removeBoxes = %d, want 9", got)
	}
	if got := removeBoxes([]int{1}); got != 1 {
		t.Fatalf("removeBoxes = %d, want 1", got)
	}
}
