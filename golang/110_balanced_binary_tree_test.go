package leetcode

import "testing"

func TestIsBalancedExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(9, nil, nil),
		makeTree(20, makeTree(15, nil, nil), makeTree(7, nil, nil)),
	)
	if !isBalanced(root) {
		t.Fatal("isBalanced() = false, want true")
	}
}

func TestIsBalancedExampleTwo(t *testing.T) {
	root := makeTree(1,
		makeTree(2, makeTree(4, makeTree(6, nil, nil), nil), makeTree(5, nil, nil)),
		makeTree(3, nil, nil),
	)
	if isBalanced(root) {
		t.Fatal("isBalanced() = true, want false")
	}
}
