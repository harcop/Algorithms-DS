package leetcode

import "testing"

func TestMaxPathSumExampleOne(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	got := maxPathSum(root)
	if got != 6 {
		t.Fatalf("maxPathSum() = %d, want 6", got)
	}
}

func TestMaxPathSumExampleTwo(t *testing.T) {
	root := makeTree(-10,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := maxPathSum(root)
	if got != 42 {
		t.Fatalf("maxPathSum() = %d, want 42", got)
	}
}
