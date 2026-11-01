package leetcode

import "testing"

func TestMinPatchesExamples(t *testing.T) {
	if got := minPatches([]int{1, 3}, 6); got != 1 {
		t.Fatalf("minPatches() = %d, want 1", got)
	}
	if got := minPatches([]int{1, 5, 10}, 20); got != 2 {
		t.Fatalf("minPatches() = %d, want 2", got)
	}
	if got := minPatches([]int{1, 2, 2}, 5); got != 0 {
		t.Fatalf("minPatches() = %d, want 0", got)
	}
}
