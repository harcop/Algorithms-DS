package leetcode

import "testing"

func TestFindTargetExamples(t *testing.T) {
	root := makeTree(5,
		makeTree(3, makeTree(2, nil, nil), makeTree(4, nil, nil)),
		makeTree(6, nil, makeTree(7, nil, nil)),
	)
	if !findTarget(root, 9) {
		t.Fatal("findTarget(9) = false, want true")
	}
	if findTarget(root, 28) {
		t.Fatal("findTarget(28) = true, want false")
	}
}
