package leetcode

import "testing"

func TestHIndexExampleOne(t *testing.T) {
	if got := hIndex([]int{3, 0, 6, 1, 5}); got != 3 {
		t.Fatalf("hIndex() = %d, want 3", got)
	}
}

func TestHIndexExampleTwo(t *testing.T) {
	if got := hIndex([]int{1, 3, 1}); got != 1 {
		t.Fatalf("hIndex() = %d, want 1", got)
	}
}
