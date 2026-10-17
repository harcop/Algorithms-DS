package leetcode

import "testing"

func TestSumNumbersExampleOne(t *testing.T) {
	root := makeTree(1, makeTree(2, nil, nil), makeTree(3, nil, nil))
	got := sumNumbers(root)
	if got != 25 {
		t.Fatalf("sumNumbers() = %d, want 25", got)
	}
}

func TestSumNumbersExampleTwo(t *testing.T) {
	root := makeTree(4,
		makeTree(9, makeTree(5, nil, nil), makeTree(1, nil, nil)),
		makeTree(0, nil, nil),
	)
	got := sumNumbers(root)
	if got != 1026 {
		t.Fatalf("sumNumbers() = %d, want 1026", got)
	}
}
