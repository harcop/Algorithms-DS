package leetcode

import "testing"

func TestMaximumProductExamples(t *testing.T) {
	if got := maximumProduct([]int{1, 2, 3}); got != 6 {
		t.Fatalf("maximumProduct([1,2,3]) = %d, want 6", got)
	}
	if got := maximumProduct([]int{1, 2, 3, 4}); got != 24 {
		t.Fatalf("maximumProduct([1,2,3,4]) = %d, want 24", got)
	}
	if got := maximumProduct([]int{-1, -2, -3}); got != -6 {
		t.Fatalf("maximumProduct([-1,-2,-3]) = %d, want -6", got)
	}
	if got := maximumProduct([]int{-100, -98, -1, 2, 3}); got != 29400 {
		t.Fatalf("maximumProduct() = %d, want 29400", got)
	}
}
