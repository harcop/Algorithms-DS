package leetcode

import "testing"

func TestMaximumGapExampleOne(t *testing.T) {
	got := maximumGap([]int{3, 6, 9, 1})
	if got != 3 {
		t.Fatalf("maximumGap() = %d, want 3", got)
	}
}

func TestMaximumGapExampleTwo(t *testing.T) {
	got := maximumGap([]int{10})
	if got != 0 {
		t.Fatalf("maximumGap() = %d, want 0", got)
	}
}
