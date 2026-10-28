package leetcode

import "testing"

func TestHIndexIIExampleOne(t *testing.T) {
	if got := hIndexII([]int{0, 1, 3, 5, 6}); got != 3 {
		t.Fatalf("hIndexII() = %d, want 3", got)
	}
}

func TestHIndexIIExampleTwo(t *testing.T) {
	if got := hIndexII([]int{1, 2, 100}); got != 2 {
		t.Fatalf("hIndexII() = %d, want 2", got)
	}
}
