package leetcode

import "testing"

func TestMaxProductExampleOne(t *testing.T) {
	got := maxProduct([]int{2, 3, -2, 4})
	if got != 6 {
		t.Fatalf("maxProduct() = %d, want 6", got)
	}
}

func TestMaxProductExampleTwo(t *testing.T) {
	got := maxProduct([]int{-2, 0, -1})
	if got != 0 {
		t.Fatalf("maxProduct() = %d, want 0", got)
	}
}
