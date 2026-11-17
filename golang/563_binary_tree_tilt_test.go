package leetcode

import "testing"

func TestFindTiltExamples(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	if got := findTilt(root); got != 1 {
		t.Fatalf("findTilt = %d, want 1", got)
	}
	root = makeTree(4,
		makeTree(2, makeTree(3, nil, nil), makeTree(5, nil, nil)),
		makeTree(9, nil, makeTree(7, nil, nil)),
	)
	if got := findTilt(root); got != 15 {
		t.Fatalf("findTilt = %d, want 15", got)
	}
}
