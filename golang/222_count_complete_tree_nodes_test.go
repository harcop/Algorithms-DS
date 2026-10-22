package leetcode

import "testing"

func TestCountNodesExampleOne(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, nil, nil), makeTree(5, nil, nil)),
		makeTree(3, makeTree(6, nil, nil), nil),
	)
	if got := countNodes(root); got != 6 {
		t.Fatalf("countNodes() = %d, want 6", got)
	}
}

func TestCountNodesExampleTwo(t *testing.T) {
	if got := countNodes(nil); got != 0 {
		t.Fatalf("countNodes() = %d, want 0", got)
	}
}

func TestCountNodesExampleThree(t *testing.T) {
	if got := countNodes(makeTree(1, nil, nil)); got != 1 {
		t.Fatalf("countNodes() = %d, want 1", got)
	}
}
