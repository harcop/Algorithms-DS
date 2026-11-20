package leetcode

import "testing"

func TestMergeTreesExample(t *testing.T) {
	root1 := makeTree(1, makeTree(3, makeTree(5, nil, nil), nil), makeTree(2, nil, nil))
	root2 := makeTree(2, makeTree(1, nil, makeTree(4, nil, nil)), makeTree(3, nil, makeTree(7, nil, nil)))
	want := makeTree(3,
		makeTree(4, makeTree(5, nil, nil), makeTree(4, nil, nil)),
		makeTree(5, nil, makeTree(7, nil, nil)),
	)
	if !isSameTree(mergeTrees(root1, root2), want) {
		t.Fatal("mergeTrees() does not match the summed tree")
	}
}
