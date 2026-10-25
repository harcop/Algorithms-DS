package leetcode

import "testing"

func exampleAncestorTree() *TreeNode {
	return makeTree(3,
		makeTree(5,
			makeTree(6, nil, nil),
			makeTree(2, makeTree(7, nil, nil), makeTree(4, nil, nil)),
		),
		makeTree(1, makeTree(0, nil, nil), makeTree(8, nil, nil)),
	)
}

func TestLowestCommonAncestorIIExampleOne(t *testing.T) {
	if got := lowestCommonAncestorII(exampleAncestorTree(), 5, 1); got != 3 {
		t.Fatalf("lowestCommonAncestorII() = %d, want 3", got)
	}
}

func TestLowestCommonAncestorIIExampleTwo(t *testing.T) {
	if got := lowestCommonAncestorII(exampleAncestorTree(), 5, 4); got != 5 {
		t.Fatalf("lowestCommonAncestorII() = %d, want 5", got)
	}
}
