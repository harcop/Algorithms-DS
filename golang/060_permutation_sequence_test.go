package leetcode

import "testing"

func TestGetPermutationExampleOne(t *testing.T) {
	got := getPermutation(3, 3)
	if got != "213" {
		t.Fatalf("getPermutation() = %q, want %q", got, "213")
	}
}

func TestGetPermutationExampleTwo(t *testing.T) {
	got := getPermutation(4, 9)
	if got != "2314" {
		t.Fatalf("getPermutation() = %q, want %q", got, "2314")
	}
}
