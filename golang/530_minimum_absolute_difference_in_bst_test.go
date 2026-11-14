package leetcode

import "testing"

func TestGetMinimumDifferenceExamples(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(6, nil, nil),
	)
	if got := getMinimumDifference(root); got != 1 {
		t.Fatalf("getMinimumDifference() = %d, want 1", got)
	}
	root = makeTree(1,
		makeTree(0, nil, nil),
		makeTree(48, makeTree(12, nil, nil), makeTree(49, nil, nil)),
	)
	if got := getMinimumDifference(root); got != 1 {
		t.Fatalf("getMinimumDifference() = %d, want 1", got)
	}
}
