package leetcode

import "testing"

func TestConvertBSTExample(t *testing.T) {
	root := makeTree(4,
		makeTree(1, makeTree(0, nil, nil), makeTree(2, nil, makeTree(3, nil, nil))),
		makeTree(6, makeTree(5, nil, nil), makeTree(7, nil, makeTree(8, nil, nil))),
	)
	got := inorderTraversal(convertBST(root))
	want := []int{36, 36, 35, 33, 30, 26, 21, 15, 8}
	if !sameInts(got, want) {
		t.Fatalf("convertBST() inorder = %v, want %v", got, want)
	}
}
