package leetcode

import "testing"

func TestSmallestFactorizationExamples(t *testing.T) {
	if got := smallestFactorization(48); got != 68 {
		t.Fatalf("smallestFactorization(48) = %d, want 68", got)
	}
	if got := smallestFactorization(15); got != 35 {
		t.Fatalf("smallestFactorization(15) = %d, want 35", got)
	}
	if got := smallestFactorization(13); got != 0 {
		t.Fatalf("smallestFactorization(13) = %d, want 0", got)
	}
}
