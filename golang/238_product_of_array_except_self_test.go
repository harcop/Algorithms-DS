package leetcode

import "testing"

func TestProductExceptSelfExampleOne(t *testing.T) {
	got := productExceptSelf([]int{1, 2, 3, 4})
	want := []int{24, 12, 8, 6}
	if !sameInts(got, want) {
		t.Fatalf("productExceptSelf() = %v, want %v", got, want)
	}
}

func TestProductExceptSelfExampleTwo(t *testing.T) {
	got := productExceptSelf([]int{-1, 1, 0, -3, 3})
	want := []int{0, 0, 9, 0, 0}
	if !sameInts(got, want) {
		t.Fatalf("productExceptSelf() = %v, want %v", got, want)
	}
}
