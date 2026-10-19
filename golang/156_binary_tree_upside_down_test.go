package leetcode

import "testing"

func TestUpsideDownBinaryTreeExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), makeTree(5, nil, nil)),
		makeTree(3, nil, nil),
	)
	got := preorderTraversal(upsideDownBinaryTree(root))
	want := []int{4, 5, 2, 3, 1}
	if !sameInts(got, want) {
		t.Fatalf("upsideDownBinaryTree() preorder = %v, want %v", got, want)
	}
}
