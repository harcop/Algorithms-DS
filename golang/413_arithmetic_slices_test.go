package leetcode

import "testing"

func TestNumberOfArithmeticSlicesExamples(t *testing.T) {
	if got := numberOfArithmeticSlices([]int{1, 2, 3, 4}); got != 3 {
		t.Fatalf("numberOfArithmeticSlices() = %d, want 3", got)
	}
	if got := numberOfArithmeticSlices([]int{1}); got != 0 {
		t.Fatalf("numberOfArithmeticSlices() = %d, want 0", got)
	}
}
