package leetcode

import "testing"

func TestLowestCommonAncestorExampleOne(t *testing.T) {
	p := makeTree(2,
		makeTree(0, nil, nil),
		makeTree(4, makeTree(3, nil, nil), makeTree(5, nil, nil)),
	)
	q := makeTree(8, makeTree(7, nil, nil), makeTree(9, nil, nil))
	root := makeTree(6, p, q)
	got := lowestCommonAncestor(root, p, q)
	if got == nil || got.Val != 6 {
		t.Fatalf("lowestCommonAncestor() = %v, want the node with value 6", got)
	}
}
