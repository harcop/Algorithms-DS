package leetcode

import "testing"

func TestIsSubtreeExamples(t *testing.T) {
	root := makeTree(3,
		makeTree(4, makeTree(1, nil, nil), makeTree(2, nil, nil)),
		makeTree(5, nil, nil),
	)
	sub := makeTree(4, makeTree(1, nil, nil), makeTree(2, nil, nil))
	if !isSubtree(root, sub) {
		t.Fatal("isSubtree = false, want true")
	}

	root = makeTree(3,
		makeTree(4, makeTree(1, nil, nil), makeTree(2, makeTree(0, nil, nil), nil)),
		makeTree(5, nil, nil),
	)
	if isSubtree(root, sub) {
		t.Fatal("isSubtree = true, want false")
	}
}
