package leetcode

import "testing"

func TestPathSumExampleOne(t *testing.T) {
	root := makeTree(5,
		makeTree(4, makeTree(11, makeTree(7, nil, nil), makeTree(2, nil, nil)), nil),
		makeTree(8, makeTree(13, nil, nil), makeTree(4, makeTree(5, nil, nil), makeTree(1, nil, nil))),
	)
	got := pathSum(root, 22)
	want := [][]int{{5, 4, 11, 2}, {5, 8, 4, 5}}
	if !sameIntGroups(got, want) {
		t.Fatalf("pathSum() = %v, want %v", got, want)
	}
}

func TestPathSumExampleTwo(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), nil)
	if len(pathSum(root, 0)) != 0 {
		t.Fatal("pathSum() found a path, want none")
	}
}
