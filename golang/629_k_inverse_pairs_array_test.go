package leetcode

import "testing"

func TestKInversePairsExamples(t *testing.T) {
	if got := kInversePairs(3, 0); got != 1 {
		t.Fatalf("kInversePairs(3, 0) = %d, want 1", got)
	}
	if got := kInversePairs(3, 1); got != 2 {
		t.Fatalf("kInversePairs(3, 1) = %d, want 2", got)
	}
	if got := kInversePairs(3, 3); got != 1 {
		t.Fatalf("kInversePairs(3, 3) = %d, want 1", got)
	}
}
