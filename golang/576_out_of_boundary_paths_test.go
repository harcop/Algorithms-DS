package leetcode

import "testing"

func TestFindPathsExamples(t *testing.T) {
	if got := findPaths(2, 2, 2, 0, 0); got != 6 {
		t.Fatalf("findPaths = %d, want 6", got)
	}
	if got := findPaths(1, 3, 3, 0, 1); got != 12 {
		t.Fatalf("findPaths = %d, want 12", got)
	}
}
