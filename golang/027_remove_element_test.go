package leetcode

import (
	"sort"
	"testing"
)

func TestRemoveElementExampleOne(t *testing.T) {
	nums := []int{3, 2, 2, 3}
	k := removeElement(nums, 3)
	if k != 2 || !sameInts(nums[:k], []int{2, 2}) {
		t.Fatalf("removeElement() = %d, nums = %v, want k=2 and [2 2]", k, nums[:k])
	}
}

func TestRemoveElementExampleTwo(t *testing.T) {
	nums := []int{0, 1, 2, 2, 3, 0, 4, 2}
	k := removeElement(nums, 2)
	got := append([]int(nil), nums[:k]...)
	sort.Ints(got)
	want := []int{0, 0, 1, 3, 4}
	if k != 5 || !sameInts(got, want) {
		t.Fatalf("removeElement() = %d, nums = %v, want k=5 and %v", k, got, want)
	}
}
