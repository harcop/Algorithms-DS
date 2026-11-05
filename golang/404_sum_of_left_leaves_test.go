package leetcode

import "testing"

func TestSumOfLeftLeavesExample(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	if got := sumOfLeftLeaves(root); got != 24 {
		t.Fatalf("sumOfLeftLeaves() = %d, want 24", got)
	}
	if got := sumOfLeftLeaves(makeTree(1, nil, nil)); got != 0 {
		t.Fatalf("sumOfLeftLeaves(leaf) = %d, want 0", got)
	}
}
