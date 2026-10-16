package leetcode

import "testing"

func TestSortColorsExampleOne(t *testing.T) {
	nums := []int{2, 0, 2, 1, 1, 0}
	sortColors(nums)
	want := []int{0, 0, 1, 1, 2, 2}
	if !sameInts(nums, want) {
		t.Fatalf("sortColors() = %v, want %v", nums, want)
	}
}

func TestSortColorsExampleTwo(t *testing.T) {
	nums := []int{2, 0, 1}
	sortColors(nums)
	want := []int{0, 1, 2}
	if !sameInts(nums, want) {
		t.Fatalf("sortColors() = %v, want %v", nums, want)
	}
}
