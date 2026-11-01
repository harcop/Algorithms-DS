package leetcode

import "testing"

func TestRobIIIExampleOne(t *testing.T) {
	root := makeTree(3,
		makeTree(2, nil, makeTree(3, nil, nil)),
		makeTree(3, nil, makeTree(1, nil, nil)),
	)
	if got := robIII(root); got != 7 {
		t.Fatalf("robIII() = %d, want 7", got)
	}
}
