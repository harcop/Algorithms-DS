package leetcode

import "testing"

func TestMaxAreaExampleOne(t *testing.T) {
	got := maxArea([]int{1, 8, 6, 2, 5, 4, 8, 3, 7})
	if got != 49 {
		t.Fatalf("maxArea() = %d, want 49", got)
	}
}

func TestMaxAreaExampleTwo(t *testing.T) {
	got := maxArea([]int{1, 1})
	if got != 1 {
		t.Fatalf("maxArea() = %d, want 1", got)
	}
}
