package leetcode

import "testing"

func TestTwoSumExampleOne(t *testing.T) {
	got := twoSum([]int{2, 7, 11, 15}, 9)
	want := []int{0, 1}
	if !sameInts(got, want) {
		t.Fatalf("twoSum() = %v, want %v", got, want)
	}
}

func TestTwoSumExampleTwo(t *testing.T) {
	got := twoSum([]int{3, 2, 4}, 6)
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("twoSum() = %v, want %v", got, want)
	}
}

func TestTwoSumExampleThree(t *testing.T) {
	got := twoSum([]int{3, 3}, 6)
	want := []int{0, 1}
	if !sameInts(got, want) {
		t.Fatalf("twoSum() = %v, want %v", got, want)
	}
}

func sameInts(a, b []int) bool {
	if len(a) != len(b) {
		return false
	}
	for i := range a {
		if a[i] != b[i] {
			return false
		}
	}
	return true
}
