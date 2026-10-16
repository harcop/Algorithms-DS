package leetcode

import "testing"

func TestLevelOrderBottomExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := levelOrderBottom(root)
	want := [][]int{{15, 7}, {9, 20}, {3}}
	if !sameMatrix(got, want) {
		t.Fatalf("levelOrderBottom() = %v, want %v", got, want)
	}
}

func TestLevelOrderBottomExampleTwo(t *testing.T) {
	if len(levelOrderBottom(nil)) != 0 {
		t.Fatal("levelOrderBottom() = levels, want none")
	}
}
