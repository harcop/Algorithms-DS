package leetcode

import (
	"math"
	"testing"
)

func TestFindMaxAverageIIExamples(t *testing.T) {
	got := findMaxAverageII([]int{1, 12, -5, -6, 50, 3}, 4)
	if math.Abs(got-12.75) > 1e-5 {
		t.Fatalf("findMaxAverageII() = %v, want 12.75", got)
	}
	got = findMaxAverageII([]int{3, -1, 3}, 2)
	if math.Abs(got-5.0/3.0) > 1e-5 {
		t.Fatalf("findMaxAverageII() = %v, want 5/3", got)
	}
}
