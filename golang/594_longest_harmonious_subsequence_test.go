package leetcode

import "testing"

func TestFindLHSExamples(t *testing.T) {
	if got := findLHS([]int{1, 3, 2, 2, 5, 2, 3, 7}); got != 5 {
		t.Fatalf("findLHS = %d, want 5", got)
	}
	if got := findLHS([]int{1, 2, 3, 4}); got != 2 {
		t.Fatalf("findLHS = %d, want 2", got)
	}
	if got := findLHS([]int{1, 1, 1, 1}); got != 0 {
		t.Fatalf("findLHS = %d, want 0", got)
	}
}
