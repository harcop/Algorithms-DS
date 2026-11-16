package leetcode

import "testing"

func TestDiameterOfBinaryTreeExample(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), makeTree(5, nil, nil)),
		makeTree(3, nil, nil),
	)
	if got := diameterOfBinaryTree(root); got != 3 {
		t.Fatalf("diameterOfBinaryTree = %d, want 3", got)
	}
}
