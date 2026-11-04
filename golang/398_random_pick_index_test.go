package leetcode

import "testing"

func TestRandomPickExample(t *testing.T) {
	picker := NewRandomPick([]int{1, 2, 3, 3, 3})
	if got := picker.Pick(1); got != 0 {
		t.Fatalf("Pick(1) = %d, want 0", got)
	}
	got := picker.Pick(3)
	if got < 2 || got > 4 {
		t.Fatalf("Pick(3) = %d, want 2, 3, or 4", got)
	}
}
