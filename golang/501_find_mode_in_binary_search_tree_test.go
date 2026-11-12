package leetcode

import (
	"sort"
	"testing"
)

func TestFindModeExamples(t *testing.T) {
	root := makeTree(1, nil, makeTree(2, makeTree(2, nil, nil), nil))
	if got := findMode(root); !sameInts(got, []int{2}) {
		t.Fatalf("findMode(repeated) = %v, want [2]", got)
	}
	got := findMode(makeTree(1, nil, makeTree(2, nil, nil)))
	sort.Ints(got)
	if !sameInts(got, []int{1, 2}) {
		t.Fatalf("findMode(unique) = %v, want [1 2]", got)
	}
}
