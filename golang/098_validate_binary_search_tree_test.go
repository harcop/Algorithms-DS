package leetcode

import "testing"

func TestIsValidBSTExampleOne(t *testing.T) {
	root := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	if !isValidBST(root) {
		t.Fatal("isValidBST() = false, want true")
	}
}

func TestIsValidBSTExampleTwo(t *testing.T) {
	root := makeTree(5,
		makeTree(1, nil, nil),
		makeTree(4, makeTree(3, nil, nil), makeTree(6, nil, nil)),
	)
	if isValidBST(root) {
		t.Fatal("isValidBST() = true, want false")
	}
}
