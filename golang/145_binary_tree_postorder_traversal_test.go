package leetcode

import "testing"

func TestPostorderTraversalExampleOne(t *testing.T) {
	root := makeTree(1, nil, makeTree(2, makeTree(3, nil, nil), nil))
	got := postorderTraversal(root)
	want := []int{3, 2, 1}
	if !sameInts(got, want) {
		t.Fatalf("postorderTraversal() = %v, want %v", got, want)
	}
}

func TestPostorderTraversalExampleTwo(t *testing.T) {
	if len(postorderTraversal(nil)) != 0 {
		t.Fatal("postorderTraversal() returned values, want none")
	}
}
