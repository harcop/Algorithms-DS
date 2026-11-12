package leetcode

import "testing"

func TestLargestValuesExamples(t *testing.T) {
	root := makeTree(1,
		makeTree(3, makeTree(5, nil, nil), makeTree(3, nil, nil)),
		makeTree(2, nil, makeTree(9, nil, nil)),
	)
	if got := largestValues(root); !sameInts(got, []int{1, 3, 9}) {
		t.Fatalf("largestValues() = %v, want [1 3 9]", got)
	}
	if largestValues(nil) != nil {
		t.Fatal("largestValues(nil) != nil")
	}
}
