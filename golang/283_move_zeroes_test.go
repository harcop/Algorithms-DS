package leetcode

import "testing"

func TestMoveZeroesExampleOne(t *testing.T) {
	nums := []int{0, 1, 0, 3, 12}
	moveZeroes(nums)
	want := []int{1, 3, 12, 0, 0}
	if !sameInts(nums, want) {
		t.Fatalf("moveZeroes() = %v, want %v", nums, want)
	}
}
