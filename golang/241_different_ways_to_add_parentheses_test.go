package leetcode

import (
	"sort"
	"testing"
)

func TestDiffWaysToComputeExampleOne(t *testing.T) {
	got := diffWaysToCompute("2-1-1")
	sort.Ints(got)
	want := []int{0, 2}
	if !sameInts(got, want) {
		t.Fatalf("diffWaysToCompute() = %v, want %v", got, want)
	}
}

func TestDiffWaysToComputeExampleTwo(t *testing.T) {
	got := diffWaysToCompute("2*3-4*5")
	sort.Ints(got)
	want := []int{-34, -14, -10, -10, 10}
	if !sameInts(got, want) {
		t.Fatalf("diffWaysToCompute() = %v, want %v", got, want)
	}
}
