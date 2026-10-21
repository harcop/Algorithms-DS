package leetcode

import "testing"

func TestRemoveElementsExampleOne(t *testing.T) {
	got := listToVec(removeElements(vecToList([]int{1, 2, 6, 3, 4, 5, 6}), 6))
	want := []int{1, 2, 3, 4, 5}
	if !sameInts(got, want) {
		t.Fatalf("removeElements() = %v, want %v", got, want)
	}
}

func TestRemoveElementsExampleTwo(t *testing.T) {
	if removeElements(nil, 1) != nil {
		t.Fatal("removeElements() returned a node, want none")
	}
}
