package leetcode

import "testing"

func TestSortedArrayToBSTExampleOne(t *testing.T) {
	root := sortedArrayToBST([]int{-10, -3, 0, 5, 9})
	if root == nil || root.Val != 0 {
		t.Fatalf("sortedArrayToBST() = %+v, want root 0", root)
	}
}

func TestSortedArrayToBSTExampleTwo(t *testing.T) {
	root := sortedArrayToBST([]int{1, 3})
	if root == nil {
		t.Fatal("sortedArrayToBST() = nil, want a tree")
	}
}
