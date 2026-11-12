package leetcode

import "testing"

func TestInorderSuccessorIIExamples(t *testing.T) {
	left := &ParentTreeNode{Val: 1}
	right := &ParentTreeNode{Val: 3}
	root := &ParentTreeNode{Val: 2, Left: left, Right: right}
	left.Parent = root
	right.Parent = root
	if got := inorderSuccessorII(left); got != root {
		t.Fatalf("successor of 1 = %v, want 2", got)
	}
	if got := inorderSuccessorII(root); got != right {
		t.Fatalf("successor of 2 = %v, want 3", got)
	}
	if got := inorderSuccessorII(right); got != nil {
		t.Fatalf("successor of 3 = %v, want nil", got)
	}
}
