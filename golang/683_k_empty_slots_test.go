package leetcode

import "testing"

func TestKEmptySlotsExamples(t *testing.T) {
	if got := kEmptySlots([]int{1, 3, 2}, 1); got != 2 {
		t.Fatalf("kEmptySlots() = %d, want 2", got)
	}
	if got := kEmptySlots([]int{1, 2, 3}, 1); got != -1 {
		t.Fatalf("kEmptySlots() = %d, want -1", got)
	}
}
