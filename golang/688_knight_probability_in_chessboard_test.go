package leetcode

import (
	"math"
	"testing"
)

func TestKnightProbabilityExamples(t *testing.T) {
	got := knightProbability(3, 2, 0, 0)
	if math.Abs(got-0.0625) > 1e-6 {
		t.Fatalf("knightProbability() = %v, want 0.0625", got)
	}
	if got := knightProbability(1, 0, 0, 0); got != 1 {
		t.Fatalf("knightProbability() = %v, want 1", got)
	}
}
