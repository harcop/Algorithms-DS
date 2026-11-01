package leetcode

import "testing"

func TestOddEvenListExampleOne(t *testing.T) {
	got := listToVec(oddEvenList(vecToList([]int{1, 2, 3, 4, 5})))
	want := []int{1, 3, 5, 2, 4}
	if !sameInts(got, want) {
		t.Fatalf("oddEvenList() = %v, want %v", got, want)
	}
}
