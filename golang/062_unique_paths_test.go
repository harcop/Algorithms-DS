package leetcode

import "testing"

func TestUniquePathsExampleOne(t *testing.T) {
	got := uniquePaths(3, 7)
	if got != 28 {
		t.Fatalf("uniquePaths() = %d, want 28", got)
	}
}

func TestUniquePathsExampleTwo(t *testing.T) {
	got := uniquePaths(3, 2)
	if got != 3 {
		t.Fatalf("uniquePaths() = %d, want 3", got)
	}
}
