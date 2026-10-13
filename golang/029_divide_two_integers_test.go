package leetcode

import (
	"math"
	"testing"
)

func TestDivideExampleOne(t *testing.T) {
	if got := divide(10, 3); got != 3 {
		t.Fatalf("divide(10, 3) = %d, want 3", got)
	}
}

func TestDivideExampleTwo(t *testing.T) {
	if got := divide(7, -3); got != -2 {
		t.Fatalf("divide(7, -3) = %d, want -2", got)
	}
}

func TestDivideOverflow(t *testing.T) {
	if got := divide(math.MinInt32, -1); got != math.MaxInt32 {
		t.Fatalf("divide(MinInt32, -1) = %d, want MaxInt32", got)
	}
}
