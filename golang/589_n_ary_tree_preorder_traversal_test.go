package leetcode

import "testing"

func TestPreorderExample(t *testing.T) {
	root := &NaryNode{Val: 1, Children: []*NaryNode{
		{Val: 3, Children: []*NaryNode{{Val: 5}, {Val: 6}}},
		{Val: 2},
		{Val: 4},
	}}
	got := preorder(root)
	want := []int{1, 3, 5, 6, 2, 4}
	if !sameInts(got, want) {
		t.Fatalf("preorder = %v, want %v", got, want)
	}
	if preorder(nil) != nil {
		t.Fatal("preorder(nil) should be empty")
	}
}
