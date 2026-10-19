package leetcode

import "testing"

func TestFindMinIIExampleOne(t *testing.T) {
	got := findMinII([]int{1, 3, 5})
	if got != 1 {
		t.Fatalf("findMinII() = %d, want 1", got)
	}
}

func TestFindMinIIExampleTwo(t *testing.T) {
	got := findMinII([]int{2, 2, 2, 0, 1})
	if got != 0 {
		t.Fatalf("findMinII() = %d, want 0", got)
	}
}
