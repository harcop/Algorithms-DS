package leetcode

import "testing"

func TestNumberOfArithmeticSlicesIIExample(t *testing.T) {
	if got := numberOfArithmeticSlicesII([]int{2, 4, 6, 8, 10}); got != 7 {
		t.Fatalf("numberOfArithmeticSlicesII() = %d, want 7", got)
	}
}
