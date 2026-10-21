package leetcode

import "testing"

func TestReverseListExampleOne(t *testing.T) {
	got := listToVec(reverseList(vecToList([]int{1, 2, 3, 4, 5})))
	want := []int{5, 4, 3, 2, 1}
	if !sameInts(got, want) {
		t.Fatalf("reverseList() = %v, want %v", got, want)
	}
}

func TestReverseListExampleTwo(t *testing.T) {
	got := listToVec(reverseList(vecToList([]int{1, 2})))
	want := []int{2, 1}
	if !sameInts(got, want) {
		t.Fatalf("reverseList() = %v, want %v", got, want)
	}
}
