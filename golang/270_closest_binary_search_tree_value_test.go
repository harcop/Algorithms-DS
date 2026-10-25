package leetcode

import "testing"

func TestClosestValueExampleOne(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(5, nil, nil),
	)
	if got := closestValue(root, 3.714286); got != 4 {
		t.Fatalf("closestValue() = %d, want 4", got)
	}
}
