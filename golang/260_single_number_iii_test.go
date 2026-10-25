package leetcode

import "testing"

func TestSingleNumberIIIExampleOne(t *testing.T) {
	got := singleNumberIII([]int{1, 2, 1, 3, 2, 5})
	want := []int{3, 5}
	if !sameInts(got, want) {
		t.Fatalf("singleNumberIII() = %v, want %v", got, want)
	}
}

func TestSingleNumberIIIExampleTwo(t *testing.T) {
	got := singleNumberIII([]int{-1, 0})
	want := []int{-1, 0}
	if !sameInts(got, want) {
		t.Fatalf("singleNumberIII() = %v, want %v", got, want)
	}
}
