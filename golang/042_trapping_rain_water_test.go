package leetcode

import "testing"

func TestTrapExampleOne(t *testing.T) {
	got := trap([]int{0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1})
	if got != 6 {
		t.Fatalf("trap() = %d, want 6", got)
	}
}

func TestTrapExampleTwo(t *testing.T) {
	got := trap([]int{4, 2, 0, 3, 2, 5})
	if got != 9 {
		t.Fatalf("trap() = %d, want 9", got)
	}
}
