package leetcode

import "testing"

func TestNextGreaterElementExamples(t *testing.T) {
	got := nextGreaterElement([]int{4, 1, 2}, []int{1, 3, 4, 2})
	if !sameInts(got, []int{-1, 3, -1}) {
		t.Fatalf("nextGreaterElement() = %v, want [-1 3 -1]", got)
	}
	got = nextGreaterElement([]int{2, 4}, []int{1, 2, 3, 4})
	if !sameInts(got, []int{3, -1}) {
		t.Fatalf("nextGreaterElement() = %v, want [3 -1]", got)
	}
}
