package leetcode

import "testing"

func TestZigzagLevelOrderExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := zigzagLevelOrder(root)
	want := [][]int{{3}, {20, 9}, {15, 7}}
	if !sameMatrix(got, want) {
		t.Fatalf("zigzagLevelOrder() = %v, want %v", got, want)
	}
}

func TestZigzagLevelOrderExampleTwo(t *testing.T) {
	if len(zigzagLevelOrder(nil)) != 0 {
		t.Fatal("zigzagLevelOrder() = levels, want none")
	}
}
