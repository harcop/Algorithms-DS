package leetcode

import "testing"

func TestGenerateTreesExampleOne(t *testing.T) {
	got := generateTrees(3)
	if len(got) != 5 {
		t.Fatalf("generateTrees() = %d trees, want 5", len(got))
	}
}

func TestGenerateTreesExampleTwo(t *testing.T) {
	got := generateTrees(1)
	if len(got) != 1 {
		t.Fatalf("generateTrees() = %d trees, want 1", len(got))
	}
}
