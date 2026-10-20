package leetcode

import "testing"

func TestLargestNumberExampleOne(t *testing.T) {
	if got := largestNumber([]int{10, 2}); got != "210" {
		t.Fatalf("largestNumber() = %q, want %q", got, "210")
	}
}

func TestLargestNumberExampleTwo(t *testing.T) {
	if got := largestNumber([]int{3, 30, 34, 5, 9}); got != "9534330" {
		t.Fatalf("largestNumber() = %q, want %q", got, "9534330")
	}
}
