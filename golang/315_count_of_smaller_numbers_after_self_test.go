package leetcode

import "testing"

func TestCountSmallerExample(t *testing.T) {
	got := countSmaller([]int{5, 2, 6, 1})
	want := []int{2, 1, 1, 0}
	if !sameInts(got, want) {
		t.Fatalf("countSmaller() = %v, want %v", got, want)
	}
}

func TestCountSmallerWithDupes(t *testing.T) {
	got := countSmaller([]int{-1, -1})
	want := []int{0, 0}
	if !sameInts(got, want) {
		t.Fatalf("countSmaller() = %v, want %v", got, want)
	}
}
