package leetcode

import (
	"sort"
	"testing"
)

func TestFindFrequentTreeSumExamples(t *testing.T) {
	root := makeTree(5, makeTree(2, nil, nil), makeTree(-3, nil, nil))
	got := findFrequentTreeSum(root)
	sort.Ints(got)
	if !sameInts(got, []int{-3, 2, 4}) {
		t.Fatalf("findFrequentTreeSum(mixed) = %v, want [-3 2 4]", got)
	}
	root = makeTree(5, makeTree(2, nil, nil), makeTree(-5, nil, nil))
	if got = findFrequentTreeSum(root); !sameInts(got, []int{2}) {
		t.Fatalf("findFrequentTreeSum(repeated) = %v, want [2]", got)
	}
}
