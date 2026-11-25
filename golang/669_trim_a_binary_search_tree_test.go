package leetcode

import "testing"

func TestTrimBSTExamples(t *testing.T) {
	root := makeTree(1, makeTree(0, nil, nil), makeTree(2, nil, nil))
	got := trimBST(root, 1, 2)
	want := makeTree(1, nil, makeTree(2, nil, nil))
	if !isSameTree(got, want) {
		t.Fatal("trimBST(1, 2) does not match the trimmed tree")
	}

	root = makeTree(3,
		makeTree(0, nil, makeTree(2, makeTree(1, nil, nil), nil)),
		makeTree(4, nil, nil),
	)
	got = trimBST(root, 1, 3)
	want = makeTree(3, makeTree(2, makeTree(1, nil, nil), nil), nil)
	if !isSameTree(got, want) {
		t.Fatal("trimBST(1, 3) does not match the trimmed tree")
	}
}
