package leetcode

import "testing"

func TestPlusOneLinkedListExamples(t *testing.T) {
	got := listToVec(plusOneLinkedList(vecToList([]int{1, 2, 3})))
	if !sameInts(got, []int{1, 2, 4}) {
		t.Fatalf("plusOneLinkedList() = %v, want [1 2 4]", got)
	}
	got = listToVec(plusOneLinkedList(vecToList([]int{9, 9})))
	if !sameInts(got, []int{1, 0, 0}) {
		t.Fatalf("plusOneLinkedList() = %v, want [1 0 0]", got)
	}
}
