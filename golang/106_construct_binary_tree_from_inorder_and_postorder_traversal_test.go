package leetcode

import "testing"

func TestBuildTreeFromPostorderExampleOne(t *testing.T) {
	root := buildTreeFromPostorder([]int{9, 3, 15, 20, 7}, []int{9, 15, 7, 20, 3})
	if root == nil || root.Val != 3 {
		t.Fatalf("buildTreeFromPostorder() = %+v, want root 3", root)
	}
}

func TestBuildTreeFromPostorderExampleTwo(t *testing.T) {
	root := buildTreeFromPostorder([]int{-1}, []int{-1})
	if root == nil || root.Val != -1 {
		t.Fatalf("buildTreeFromPostorder() = %+v, want a single node -1", root)
	}
}
