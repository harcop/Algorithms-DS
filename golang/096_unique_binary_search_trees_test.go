package leetcode

import "testing"

func TestNumTreesExampleOne(t *testing.T) {
	got := numTrees(3)
	if got != 5 {
		t.Fatalf("numTrees() = %d, want 5", got)
	}
}

func TestNumTreesExampleTwo(t *testing.T) {
	got := numTrees(1)
	if got != 1 {
		t.Fatalf("numTrees() = %d, want 1", got)
	}
}
