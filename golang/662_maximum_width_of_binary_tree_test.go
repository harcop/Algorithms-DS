package leetcode

import "testing"

func TestWidthOfBinaryTreeExamples(t *testing.T) {
	root := makeTree(1,
		makeTree(3, makeTree(5, nil, nil), makeTree(3, nil, nil)),
		makeTree(2, nil, makeTree(9, nil, nil)),
	)
	if got := widthOfBinaryTree(root); got != 4 {
		t.Fatalf("widthOfBinaryTree() = %d, want 4", got)
	}
	root = makeTree(1, makeTree(3, makeTree(5, nil, nil), makeTree(3, nil, nil)), nil)
	if got := widthOfBinaryTree(root); got != 2 {
		t.Fatalf("widthOfBinaryTree() = %d, want 2", got)
	}
}
