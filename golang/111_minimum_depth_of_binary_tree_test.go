package leetcode

import "testing"

func TestMinDepthExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	got := minDepth(root)
	if got != 2 {
		t.Fatalf("minDepth() = %d, want 2", got)
	}
}

func TestMinDepthExampleTwo(t *testing.T) {
	root := makeTree(2, nil, makeTree(3, nil, makeTree(4, nil, makeTree(5, nil, makeTree(6, nil, nil)))))
	got := minDepth(root)
	if got != 5 {
		t.Fatalf("minDepth() = %d, want 5", got)
	}
}
