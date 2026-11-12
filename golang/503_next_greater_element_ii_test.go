package leetcode

import "testing"

func TestNextGreaterElementsExamples(t *testing.T) {
	if got := nextGreaterElements([]int{1, 2, 1}); !sameInts(got, []int{2, -1, 2}) {
		t.Fatalf("nextGreaterElements([1 2 1]) = %v, want [2 -1 2]", got)
	}
	if got := nextGreaterElements([]int{1, 2, 3, 4, 3}); !sameInts(got, []int{2, 3, 4, -1, 4}) {
		t.Fatalf("nextGreaterElements([1 2 3 4 3]) = %v, want [2 3 4 -1 4]", got)
	}
}
