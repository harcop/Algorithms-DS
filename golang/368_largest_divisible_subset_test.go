package leetcode

import "testing"

func TestLargestDivisibleSubsetExamples(t *testing.T) {
	got := largestDivisibleSubset([]int{1, 2, 3})
	want := []int{1, 2}
	if !sameInts(got, want) {
		t.Fatalf("largestDivisibleSubset() = %v, want %v", got, want)
	}
	got = largestDivisibleSubset([]int{1, 2, 4, 8})
	want = []int{1, 2, 4, 8}
	if !sameInts(got, want) {
		t.Fatalf("largestDivisibleSubset() = %v, want %v", got, want)
	}
}
