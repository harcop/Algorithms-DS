package leetcode

import "testing"

func TestCandyExampleOne(t *testing.T) {
	got := candy([]int{1, 0, 2})
	if got != 5 {
		t.Fatalf("candy() = %d, want 5", got)
	}
}

func TestCandyExampleTwo(t *testing.T) {
	got := candy([]int{1, 2, 2})
	if got != 4 {
		t.Fatalf("candy() = %d, want 4", got)
	}
}
