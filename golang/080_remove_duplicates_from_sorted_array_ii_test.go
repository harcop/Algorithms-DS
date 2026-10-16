package leetcode

import "testing"

func TestRemoveDuplicatesIIExampleOne(t *testing.T) {
	nums := []int{1, 1, 1, 2, 2, 3}
	k := removeDuplicatesII(nums)
	want := []int{1, 1, 2, 2, 3}
	if k != len(want) || !sameInts(nums[:k], want) {
		t.Fatalf("removeDuplicatesII() = %d, %v, want %v", k, nums[:k], want)
	}
}

func TestRemoveDuplicatesIIExampleTwo(t *testing.T) {
	nums := []int{0, 0, 1, 1, 1, 1, 2, 3, 3}
	k := removeDuplicatesII(nums)
	want := []int{0, 0, 1, 1, 2, 3, 3}
	if k != len(want) || !sameInts(nums[:k], want) {
		t.Fatalf("removeDuplicatesII() = %d, %v, want %v", k, nums[:k], want)
	}
}
