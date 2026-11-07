package leetcode

import "testing"

func TestPathSumIIIExample(t *testing.T) {
	root := makeTree(10,
		makeTree(5,
			makeTree(3, makeTree(3, nil, nil), makeTree(-2, nil, nil)),
			makeTree(2, nil, makeTree(1, nil, nil)),
		),
		makeTree(-3, nil, makeTree(11, nil, nil)),
	)
	if got := pathSumIII(root, 8); got != 3 {
		t.Fatalf("pathSumIII() = %d, want 3", got)
	}
	if got := pathSumIII(nil, 1); got != 0 {
		t.Fatalf("pathSumIII(nil) = %d, want 0", got)
	}
}
