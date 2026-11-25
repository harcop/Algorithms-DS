package leetcode

import "testing"

func TestMaximumSwapExamples(t *testing.T) {
	if got := maximumSwap(2736); got != 7236 {
		t.Fatalf("maximumSwap(2736) = %d, want 7236", got)
	}
	if got := maximumSwap(9973); got != 9973 {
		t.Fatalf("maximumSwap(9973) = %d, want 9973", got)
	}
}
