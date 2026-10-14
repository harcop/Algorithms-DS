package leetcode

import "testing"

func TestNextPermutationExampleOne(t *testing.T) {
	nums := []int{1, 2, 3}
	nextPermutation(nums)
	want := []int{1, 3, 2}
	if !sameInts(nums, want) {
		t.Fatalf("nextPermutation() = %v, want %v", nums, want)
	}
}

func TestNextPermutationExampleTwo(t *testing.T) {
	nums := []int{3, 2, 1}
	nextPermutation(nums)
	want := []int{1, 2, 3}
	if !sameInts(nums, want) {
		t.Fatalf("nextPermutation() = %v, want %v", nums, want)
	}
}

func TestNextPermutationExampleThree(t *testing.T) {
	nums := []int{1, 1, 5}
	nextPermutation(nums)
	want := []int{1, 5, 1}
	if !sameInts(nums, want) {
		t.Fatalf("nextPermutation() = %v, want %v", nums, want)
	}
}
