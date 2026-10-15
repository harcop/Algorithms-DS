package leetcode

import "testing"

func TestRotateRightExampleOne(t *testing.T) {
	got := listToVec(rotateRight(vecToList([]int{1, 2, 3, 4, 5}), 2))
	want := []int{4, 5, 1, 2, 3}
	if !sameInts(got, want) {
		t.Fatalf("rotateRight() = %v, want %v", got, want)
	}
}

func TestRotateRightExampleTwo(t *testing.T) {
	got := listToVec(rotateRight(vecToList([]int{0, 1, 2}), 4))
	want := []int{2, 0, 1}
	if !sameInts(got, want) {
		t.Fatalf("rotateRight() = %v, want %v", got, want)
	}
}
