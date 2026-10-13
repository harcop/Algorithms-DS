package leetcode

import "testing"

func TestReverseKGroupExampleOne(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4, 5})
	got := listToVec(reverseKGroup(head, 2))
	want := []int{2, 1, 4, 3, 5}
	if !sameInts(got, want) {
		t.Fatalf("reverseKGroup() = %v, want %v", got, want)
	}
}

func TestReverseKGroupExampleTwo(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4, 5})
	got := listToVec(reverseKGroup(head, 3))
	want := []int{3, 2, 1, 4, 5}
	if !sameInts(got, want) {
		t.Fatalf("reverseKGroup() = %v, want %v", got, want)
	}
}
