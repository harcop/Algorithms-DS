package leetcode

import "testing"

func TestCheckEqualTreeExamples(t *testing.T) {
	root := makeTree(5,
		makeTree(10, nil, nil),
		makeTree(10, makeTree(2, nil, nil), makeTree(3, nil, nil)),
	)
	if !checkEqualTree(root) {
		t.Fatal("checkEqualTree() = false, want true")
	}
	root = makeTree(1,
		makeTree(2, nil, nil),
		makeTree(10, makeTree(2, nil, nil), makeTree(20, nil, nil)),
	)
	if checkEqualTree(root) {
		t.Fatal("checkEqualTree() = true, want false")
	}
}
