package leetcode

import "testing"

func TestFindMedianOdd(t *testing.T) {
	if got := findMedian([]int{1, 2, 3}); got != 2 {
		t.Fatalf("findMedian() = %v, want 2", got)
	}
}

func TestFindMedianEven(t *testing.T) {
	if got := findMedian([]int{1, 2, 3, 4}); got != 2.5 {
		t.Fatalf("findMedian() = %v, want 2.5", got)
	}
}

func TestFindMedianUnsorted(t *testing.T) {
	if got := findMedian([]int{3, 1, 2}); got != 2 {
		t.Fatalf("findMedian() = %v, want 2", got)
	}
	if got := findMedian([]int{4, 1, 3, 2}); got != 2.5 {
		t.Fatalf("findMedian() = %v, want 2.5", got)
	}
}
