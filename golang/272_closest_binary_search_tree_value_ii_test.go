package leetcode

import (
	"sort"
	"testing"
)

func TestClosestKValuesExampleOne(t *testing.T) {
	root := makeTree(4,
		makeTree(2, makeTree(1, nil, nil), makeTree(3, nil, nil)),
		makeTree(6, makeTree(5, nil, nil), makeTree(7, nil, nil)),
	)
	got := closestKValues(root, 3.8, 2)
	sort.Ints(got)
	want := []int{3, 4}
	if !sameInts(got, want) {
		t.Fatalf("closestKValues() = %v, want %v", got, want)
	}
}
