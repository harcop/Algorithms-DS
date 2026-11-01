package leetcode

import "testing"

func TestLargestBSTSubtreeExampleOne(t *testing.T) {
	root := makeTree(10,
		makeTree(5, nil, nil),
		makeTree(15, makeTree(14, nil, nil), makeTree(18, nil, nil)),
	)
	if got := largestBSTSubtree(root); got != 5 {
		t.Fatalf("largestBSTSubtree() = %d, want 5", got)
	}
}

func TestLargestBSTSubtreeAllBST(t *testing.T) {
	root := makeTree(2, makeTree(1, nil, nil), makeTree(4, nil, nil))
	if got := largestBSTSubtree(root); got != 3 {
		t.Fatalf("largestBSTSubtree() = %d, want 3", got)
	}
}
