package leetcode

import "testing"

func TestFindMedianSortedArraysExampleOne(t *testing.T) {
	got := findMedianSortedArrays([]int{1, 3}, []int{2})
	if abs(got-2) > 1e-9 {
		t.Fatalf("findMedianSortedArrays() = %v, want 2", got)
	}
}

func TestFindMedianSortedArraysExampleTwo(t *testing.T) {
	got := findMedianSortedArrays([]int{1, 2}, []int{3, 4})
	if abs(got-2.5) > 1e-9 {
		t.Fatalf("findMedianSortedArrays() = %v, want 2.5", got)
	}
}

func abs(v float64) float64 {
	if v < 0 {
		return -v
	}
	return v
}
