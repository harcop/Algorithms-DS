package leetcode

import "testing"

func TestFindDiagonalOrderExamples(t *testing.T) {
	got := findDiagonalOrder([][]int{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}})
	if !sameInts(got, []int{1, 2, 4, 7, 5, 3, 6, 8, 9}) {
		t.Fatalf("findDiagonalOrder() = %v, want the diagonal order", got)
	}
	got = findDiagonalOrder([][]int{{1, 2}, {3, 4}})
	if !sameInts(got, []int{1, 2, 3, 4}) {
		t.Fatalf("findDiagonalOrder() = %v, want [1 2 3 4]", got)
	}
}
