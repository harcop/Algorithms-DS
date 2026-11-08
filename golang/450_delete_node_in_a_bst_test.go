package leetcode

import "testing"

func TestDeleteBSTNodeExample(t *testing.T) {
	root := makeTree(5,
		makeTree(3, makeTree(2, nil, nil), makeTree(4, nil, nil)),
		makeTree(6, nil, makeTree(7, nil, nil)),
	)
	got := inorderTraversal(deleteBSTNode(root, 3))
	want := []int{2, 4, 5, 6, 7}
	if !sameInts(got, want) {
		t.Fatalf("deleteBSTNode(3) = %v, want %v", got, want)
	}

	root = makeTree(5,
		makeTree(3, makeTree(2, nil, nil), makeTree(4, nil, nil)),
		makeTree(7, makeTree(6, nil, nil), nil),
	)
	got = inorderTraversal(deleteBSTNode(root, 5))
	want = []int{2, 3, 4, 6, 7}
	if !sameInts(got, want) {
		t.Fatalf("deleteBSTNode(5) = %v, want %v", got, want)
	}
}
