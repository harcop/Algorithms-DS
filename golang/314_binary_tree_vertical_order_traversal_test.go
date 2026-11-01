package leetcode

import "testing"

func TestVerticalOrderExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := verticalOrder(root)
	want := [][]int{{9}, {3, 15}, {20}, {7}}
	if !sameMatrix(got, want) {
		t.Fatalf("verticalOrder() = %v, want %v", got, want)
	}
}
