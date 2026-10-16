package leetcode

import "testing"

func TestLargestRectangleAreaExampleOne(t *testing.T) {
	got := largestRectangleArea([]int{2, 1, 5, 6, 2, 3})
	if got != 10 {
		t.Fatalf("largestRectangleArea() = %d, want 10", got)
	}
}

func TestLargestRectangleAreaExampleTwo(t *testing.T) {
	got := largestRectangleArea([]int{2, 4})
	if got != 4 {
		t.Fatalf("largestRectangleArea() = %d, want 4", got)
	}
}
