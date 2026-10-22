package leetcode

import "testing"

func TestInvertTreeExampleOne(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(7, makeTree(6, nil, nil), makeTree(9, nil, nil)),
	)
	got := preorderTraversal(invertTree(root))
	want := []int{4, 7, 9, 6, 2, 3, 1}
	if !sameInts(got, want) {
		t.Fatalf("invertTree() preorder = %v, want %v", got, want)
	}
}
