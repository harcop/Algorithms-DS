package leetcode

import "testing"

func TestFindBottomLeftValueExamples(t *testing.T) {
	root := makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil))
	if got := findBottomLeftValue(root); got != 1 {
		t.Fatalf("findBottomLeftValue(small) = %d, want 1", got)
	}
	root = makeTree(1,
		makeTree(2, makeTree(4, nil, nil), nil),
		makeTree(3, makeTree(5, makeTree(7, nil, nil), nil), makeTree(6, nil, nil)),
	)
	if got := findBottomLeftValue(root); got != 7 {
		t.Fatalf("findBottomLeftValue(deep) = %d, want 7", got)
	}
}
