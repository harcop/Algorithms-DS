package leetcode

import "testing"

func TestCountUnivalSubtreesExampleOne(t *testing.T) {
	root := makeTree(5,
		makeTree(1, makeTree(5, nil, nil), makeTree(5, nil, nil)),
		makeTree(5, nil, makeTree(5, nil, nil)),
	)
	if got := countUnivalSubtrees(root); got != 4 {
		t.Fatalf("countUnivalSubtrees() = %d, want 4", got)
	}
}
