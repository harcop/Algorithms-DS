package leetcode

import "testing"

func TestInorderSuccessorExampleOne(t *testing.T) {
	root := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	if got := inorderSuccessor(root, 1); got != 2 {
		t.Fatalf("inorderSuccessor() = %d, want 2", got)
	}
}
