package leetcode

import "testing"

func TestSingleNumberIIExampleOne(t *testing.T) {
	got := singleNumberII([]int{2, 2, 3, 2})
	if got != 3 {
		t.Fatalf("singleNumberII() = %d, want 3", got)
	}
}

func TestSingleNumberIIExampleTwo(t *testing.T) {
	got := singleNumberII([]int{0, 1, 0, 1, 0, 1, 99})
	if got != 99 {
		t.Fatalf("singleNumberII() = %d, want 99", got)
	}
}
