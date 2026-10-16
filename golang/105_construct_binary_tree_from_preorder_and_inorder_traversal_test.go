package leetcode

import "testing"

func TestBuildTreeExampleOne(t *testing.T) {
	root := buildTree([]int{3, 9, 20, 15, 7}, []int{9, 3, 15, 20, 7})
	if root == nil || root.Val != 3 || root.Left == nil || root.Left.Val != 9 || root.Right == nil || root.Right.Val != 20 {
		t.Fatalf("buildTree() = %+v, want root 3 with children 9 and 20", root)
	}
}

func TestBuildTreeExampleTwo(t *testing.T) {
	root := buildTree([]int{-1}, []int{-1})
	if root == nil || root.Val != -1 {
		t.Fatalf("buildTree() = %+v, want a single node -1", root)
	}
}
