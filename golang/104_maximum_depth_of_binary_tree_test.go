package leetcode

import "testing"

func TestMaxDepthExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := maxDepth(root)
	if got != 3 {
		t.Fatalf("maxDepth() = %d, want 3", got)
	}
}

func TestMaxDepthExampleTwo(t *testing.T) {
	if maxDepth(nil) != 0 {
		t.Fatal("maxDepth() != 0, want 0")
	}
}
