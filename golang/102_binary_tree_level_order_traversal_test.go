package leetcode

import "testing"

func TestLevelOrderExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := levelOrder(root)
	want := [][]int{{3}, {9, 20}, {15, 7}}
	if !sameMatrix(got, want) {
		t.Fatalf("levelOrder() = %v, want %v", got, want)
	}
}

func TestLevelOrderExampleTwo(t *testing.T) {
	if len(levelOrder(nil)) != 0 {
		t.Fatal("levelOrder() = levels, want none")
	}
}
