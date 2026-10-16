package leetcode

import "testing"

func TestMergeSortedArrayExampleOne(t *testing.T) {
	nums1 := []int{1, 2, 3, 0, 0, 0}
	mergeSortedArray(nums1, 3, []int{2, 5, 6}, 3)
	want := []int{1, 2, 2, 3, 5, 6}
	if !sameInts(nums1, want) {
		t.Fatalf("mergeSortedArray() = %v, want %v", nums1, want)
	}
}

func TestMergeSortedArrayExampleTwo(t *testing.T) {
	nums1 := []int{1}
	mergeSortedArray(nums1, 1, nil, 0)
	want := []int{1}
	if !sameInts(nums1, want) {
		t.Fatalf("mergeSortedArray() = %v, want %v", nums1, want)
	}
}

func TestMergeSortedArrayExampleThree(t *testing.T) {
	nums1 := []int{0}
	mergeSortedArray(nums1, 0, []int{1}, 1)
	want := []int{1}
	if !sameInts(nums1, want) {
		t.Fatalf("mergeSortedArray() = %v, want %v", nums1, want)
	}
}
