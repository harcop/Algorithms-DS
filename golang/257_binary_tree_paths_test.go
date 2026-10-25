package leetcode

import "testing"

func TestBinaryTreePathsExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, nil, makeTree(5, nil, nil)),
		makeTree(3, nil, nil),
	)
	got := binaryTreePaths(root)
	want := []string{"1->2->5", "1->3"}
	if !sameStrings(got, want) {
		t.Fatalf("binaryTreePaths() = %v, want %v", got, want)
	}
}
