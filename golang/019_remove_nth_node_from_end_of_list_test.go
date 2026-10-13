package leetcode

import "testing"

func TestRemoveNthFromEndExampleOne(t *testing.T) {
	head := vecToList([]int{1, 2, 3, 4, 5})
	got := listToVec(removeNthFromEnd(head, 2))
	want := []int{1, 2, 3, 5}
	if !sameInts(got, want) {
		t.Fatalf("removeNthFromEnd() = %v, want %v", got, want)
	}
}

func TestRemoveNthFromEndExampleTwo(t *testing.T) {
	head := vecToList([]int{1})
	got := listToVec(removeNthFromEnd(head, 1))
	if len(got) != 0 {
		t.Fatalf("removeNthFromEnd() = %v, want empty", got)
	}
}

func TestRemoveNthFromEndExampleThree(t *testing.T) {
	head := vecToList([]int{1, 2})
	got := listToVec(removeNthFromEnd(head, 1))
	want := []int{1}
	if !sameInts(got, want) {
		t.Fatalf("removeNthFromEnd() = %v, want %v", got, want)
	}
}
