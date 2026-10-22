package leetcode

import "testing"

func TestKthSmallestExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(1, nil, makeTree(2, nil, nil)),
		makeTree(4, nil, nil),
	)
	if got := kthSmallest(root, 1); got != 1 {
		t.Fatalf("kthSmallest() = %d, want 1", got)
	}
}

func TestKthSmallestExampleTwo(t *testing.T) {
	root := makeTree(5,
		makeTree(3,
			makeTree(2, makeTree(1, nil, nil), nil),
			makeTree(4, nil, nil),
		),
		makeTree(6, nil, nil),
	)
	if got := kthSmallest(root, 3); got != 3 {
		t.Fatalf("kthSmallest() = %d, want 3", got)
	}
}
