package leetcode

import "testing"

func TestHasPathSumExampleOne(t *testing.T) {
	root := makeTree(5,
		makeTree(4, makeTree(11, makeTree(7, nil, nil), makeTree(2, nil, nil)), nil),
		makeTree(8, makeTree(13, nil, nil), makeTree(4, nil, makeTree(1, nil, nil))),
	)
	if !hasPathSum(root, 22) {
		t.Fatal("hasPathSum() = false, want true")
	}
}

func TestHasPathSumExampleTwo(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	if hasPathSum(root, 5) {
		t.Fatal("hasPathSum() = true, want false")
	}
}
