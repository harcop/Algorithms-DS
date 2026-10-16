package leetcode

import "testing"

func TestRecoverTreeExampleOne(t *testing.T) {
	root := makeTree(1, makeTree(3, nil, nil), makeTree(2, nil, nil))
	recoverTree(root)
	got := inorderTraversal(root)
	want := []int{1, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("recoverTree() inorder = %v, want %v", got, want)
	}
}

func TestRecoverTreeExampleTwo(t *testing.T) {
	root := makeTree(3, makeTree(1, nil, nil), makeTree(4, makeTree(2, nil, nil), nil))
	recoverTree(root)
	got := inorderTraversal(root)
	want := []int{1, 2, 3, 4}
	if !sameInts(got, want) {
		t.Fatalf("recoverTree() inorder = %v, want %v", got, want)
	}
}
