package leetcode

import "testing"

func TestThirdMaxExamples(t *testing.T) {
	if got := thirdMax([]int{3, 2, 1}); got != 1 {
		t.Fatalf("thirdMax() = %d, want 1", got)
	}
	if got := thirdMax([]int{1, 2}); got != 2 {
		t.Fatalf("thirdMax() = %d, want 2", got)
	}
	if got := thirdMax([]int{2, 2, 3, 1}); got != 1 {
		t.Fatalf("thirdMax() = %d, want 1", got)
	}
}
