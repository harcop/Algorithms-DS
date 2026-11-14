package leetcode

import "testing"

func TestStr2treeExample(t *testing.T) {
	root := str2tree("4(2(3)(1))(6(5))")
	got := preorderTraversal(root)
	want := []int{4, 2, 3, 1, 6, 5}
	if !sameInts(got, want) {
		t.Fatalf("str2tree() preorder = %v, want %v", got, want)
	}
	if str2tree("") != nil {
		t.Fatal("str2tree(empty) = non-nil, want nil")
	}
}
