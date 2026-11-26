package leetcode

import "testing"

func TestSearchBSTExamples(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(7, nil, nil),
	)
	want := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	if !isSameTree(searchBST(root, 2), want) {
		t.Fatal("searchBST(2) does not match the subtree")
	}
	if searchBST(root, 5) != nil {
		t.Fatal("searchBST(5) is not nil")
	}
}
