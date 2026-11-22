package leetcode

import "testing"

func TestAddOneRowExamples(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(3, nil, nil), makeTree(1, nil, nil)),
		makeTree(6, makeTree(5, nil, nil), nil),
	)
	want := makeTree(4,
		makeTree(1, makeTree(2, makeTree(3, nil, nil), makeTree(1, nil, nil)), nil),
		makeTree(1, nil, makeTree(6, makeTree(5, nil, nil), nil)),
	)
	if !isSameTree(addOneRow(root, 1, 2), want) {
		t.Fatal("addOneRow(depth 2) does not match the expected tree")
	}

	root = makeTree(4, makeTree(2, makeTree(3, nil, nil), makeTree(1, nil, nil)), nil)
	want = makeTree(4,
		makeTree(2,
			makeTree(1, makeTree(3, nil, nil), nil),
			makeTree(1, nil, makeTree(1, nil, nil)),
		),
		nil,
	)
	if !isSameTree(addOneRow(root, 1, 3), want) {
		t.Fatal("addOneRow(depth 3) does not match the expected tree")
	}
}
