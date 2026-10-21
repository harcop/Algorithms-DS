package leetcode

import "testing"

func TestRightSideViewExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), nil),
		makeTree(3, nil, makeTree(5, nil, nil)),
	)
	got := rightSideView(root)
	want := []int{1, 3, 5}
	if !sameInts(got, want) {
		t.Fatalf("rightSideView() = %v, want %v", got, want)
	}
}
