package leetcode

import "testing"

func TestFindRightIntervalExamples(t *testing.T) {
	if got := findRightInterval([][]int{{1, 2}}); !sameInts(got, []int{-1}) {
		t.Fatalf("findRightInterval(single) = %v, want [-1]", got)
	}
	got := findRightInterval([][]int{{3, 4}, {2, 3}, {1, 2}})
	if !sameInts(got, []int{-1, 0, 1}) {
		t.Fatalf("findRightInterval() = %v, want [-1 0 1]", got)
	}
}
