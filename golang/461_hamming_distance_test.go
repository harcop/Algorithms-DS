package leetcode

import "testing"

func TestHammingDistanceExamples(t *testing.T) {
	if got := hammingDistance(1, 4); got != 2 {
		t.Fatalf("hammingDistance(1, 4) = %d, want 2", got)
	}
	if got := hammingDistance(3, 1); got != 1 {
		t.Fatalf("hammingDistance(3, 1) = %d, want 1", got)
	}
}
