package leetcode

import "testing"

func TestRemoveDuplicatesExampleOne(t *testing.T) {
	nums := []int{1, 1, 2}
	k := removeDuplicates(nums)
	if k != 2 || !sameInts(nums[:k], []int{1, 2}) {
		t.Fatalf("removeDuplicates() = %d, nums = %v, want k=2 and [1 2]", k, nums[:k])
	}
}

func TestRemoveDuplicatesExampleTwo(t *testing.T) {
	nums := []int{0, 0, 1, 1, 1, 2, 2, 3, 3, 4}
	k := removeDuplicates(nums)
	want := []int{0, 1, 2, 3, 4}
	if k != 5 || !sameInts(nums[:k], want) {
		t.Fatalf("removeDuplicates() = %d, nums = %v, want k=5 and %v", k, nums[:k], want)
	}
}
