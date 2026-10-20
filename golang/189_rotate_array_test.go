package leetcode

import "testing"

func TestRotateArrayExampleOne(t *testing.T) {
	nums := []int{1, 2, 3, 4, 5, 6, 7}
	rotateArray(nums, 3)
	want := []int{5, 6, 7, 1, 2, 3, 4}
	if !sameInts(nums, want) {
		t.Fatalf("rotateArray() = %v, want %v", nums, want)
	}
}

func TestRotateArrayExampleTwo(t *testing.T) {
	nums := []int{-1, -100, 3, 99}
	rotateArray(nums, 2)
	want := []int{3, 99, -1, -100}
	if !sameInts(nums, want) {
		t.Fatalf("rotateArray() = %v, want %v", nums, want)
	}
}
