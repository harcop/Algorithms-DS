package leetcode

import "testing"

func TestTwoSumIIExampleOne(t *testing.T) {
	got := twoSumII([]int{2, 7, 11, 15}, 9)
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("twoSumII() = %v, want %v", got, want)
	}
}

func TestTwoSumIIExampleTwo(t *testing.T) {
	got := twoSumII([]int{2, 3, 4}, 6)
	want := []int{1, 3}
	if !sameInts(got, want) {
		t.Fatalf("twoSumII() = %v, want %v", got, want)
	}
}

func TestTwoSumIIExampleThree(t *testing.T) {
	got := twoSumII([]int{-1, 0}, -1)
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("twoSumII() = %v, want %v", got, want)
	}
}
