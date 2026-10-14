package leetcode

import "testing"

func TestSearchRangeExampleOne(t *testing.T) {
	got := searchRange([]int{5, 7, 7, 8, 8, 10}, 8)
	want := []int{3, 4}
	if !sameInts(got, want) {
		t.Fatalf("searchRange() = %v, want %v", got, want)
	}
}

func TestSearchRangeExampleTwo(t *testing.T) {
	got := searchRange([]int{5, 7, 7, 8, 8, 10}, 6)
	want := []int{-1, -1}
	if !sameInts(got, want) {
		t.Fatalf("searchRange() = %v, want %v", got, want)
	}
}

func TestSearchRangeExampleThree(t *testing.T) {
	got := searchRange(nil, 0)
	want := []int{-1, -1}
	if !sameInts(got, want) {
		t.Fatalf("searchRange() = %v, want %v", got, want)
	}
}
