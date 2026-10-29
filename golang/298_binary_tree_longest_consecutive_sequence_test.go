package leetcode

import "testing"

func TestLongestConsecutiveTreeExampleOne(t *testing.T) {
	root := makeTree(1, nil, makeTree(3,
		makeTree(2, nil, nil),
		makeTree(4, nil, makeTree(5, nil, nil)),
	))
	if got := longestConsecutiveTree(root); got != 3 {
		t.Fatalf("longestConsecutiveTree() = %d, want 3", got)
	}
}
