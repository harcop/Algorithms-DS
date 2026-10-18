package leetcode

import "testing"

func TestPartitionPalindromeExampleOne(t *testing.T) {
	got := partitionPalindrome("aab")
	want := [][]string{{"a", "a", "b"}, {"aa", "b"}}
	if !sameStringPaths(got, want) {
		t.Fatalf("partitionPalindrome() = %v, want %v", got, want)
	}
}

func TestPartitionPalindromeExampleTwo(t *testing.T) {
	got := partitionPalindrome("a")
	want := [][]string{{"a"}}
	if !sameStringPaths(got, want) {
		t.Fatalf("partitionPalindrome() = %v, want %v", got, want)
	}
}
