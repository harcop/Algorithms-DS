package leetcode

import "testing"

func TestFindLeavesExample(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), makeTree(5, nil, nil)),
		makeTree(3, nil, nil),
	)
	got := findLeaves(root)
	want := [][]int{{4, 5, 3}, {2}, {1}}
	if !sameMatrix(got, want) {
		t.Fatalf("findLeaves() = %v, want %v", got, want)
	}
}
