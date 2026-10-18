package leetcode

import "testing"

func TestPreorderTraversalExampleOne(t *testing.T) {
	root := makeTree(1, nil, makeTree(2, makeTree(3, nil, nil), nil))
	got := preorderTraversal(root)
	want := []int{1, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("preorderTraversal() = %v, want %v", got, want)
	}
}

func TestPreorderTraversalExampleTwo(t *testing.T) {
	if len(preorderTraversal(nil)) != 0 {
		t.Fatal("preorderTraversal() returned values, want none")
	}
}
