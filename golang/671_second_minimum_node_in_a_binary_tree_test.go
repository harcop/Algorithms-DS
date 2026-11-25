package leetcode

import "testing"

func TestFindSecondMinimumValueExamples(t *testing.T) {
	root := makeTree(2,
		makeTree(2, nil, nil),
		makeTree(5, makeTree(5, nil, nil), makeTree(7, nil, nil)),
	)
	if got := findSecondMinimumValue(root); got != 5 {
		t.Fatalf("findSecondMinimumValue() = %d, want 5", got)
	}
	root = makeTree(2, makeTree(2, nil, nil), makeTree(2, nil, nil))
	if got := findSecondMinimumValue(root); got != -1 {
		t.Fatalf("findSecondMinimumValue() = %d, want -1", got)
	}
}
