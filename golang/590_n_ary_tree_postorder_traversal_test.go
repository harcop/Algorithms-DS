package leetcode

import "testing"

func TestPostorderExample(t *testing.T) {
	root := &NaryNode{Val: 1, Children: []*NaryNode{
		{Val: 3, Children: []*NaryNode{{Val: 5}, {Val: 6}}},
		{Val: 2},
		{Val: 4},
	}}
	got := postorder(root)
	want := []int{5, 6, 3, 2, 4, 1}
	if !sameInts(got, want) {
		t.Fatalf("postorder = %v, want %v", got, want)
	}
}
