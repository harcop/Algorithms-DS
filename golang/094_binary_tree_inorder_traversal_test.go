package leetcode

import "testing"

func TestInorderTraversalExampleOne(t *testing.T) {
	root := makeTree(1, nil, makeTree(2, makeTree(3, nil, nil), nil))
	got := inorderTraversal(root)
	want := []int{1, 3, 2}
	if !sameInts(got, want) {
		t.Fatalf("inorderTraversal() = %v, want %v", got, want)
	}
}

func TestInorderTraversalExampleTwo(t *testing.T) {
	got := inorderTraversal(nil)
	if len(got) != 0 {
		t.Fatalf("inorderTraversal() = %v, want none", got)
	}
}

func makeTree(val int, left, right *TreeNode) *TreeNode {
	return &TreeNode{Val: val, Left: left, Right: right}
}
