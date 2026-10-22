package leetcode

import "testing"

func TestComputeAreaExampleOne(t *testing.T) {
	if got := computeArea(-3, 0, 3, 4, 0, -1, 9, 2); got != 45 {
		t.Fatalf("computeArea() = %d, want 45", got)
	}
}

func TestComputeAreaExampleTwo(t *testing.T) {
	if got := computeArea(-2, -2, 2, 2, -2, -2, 2, 2); got != 16 {
		t.Fatalf("computeArea() = %d, want 16", got)
	}
}
