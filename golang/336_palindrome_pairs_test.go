package leetcode

import "testing"

func TestPalindromePairsExample(t *testing.T) {
	got := palindromePairs([]string{"abcd", "dcba", "lls", "s", "sssll"})
	want := [][]int{{0, 1}, {1, 0}, {3, 2}, {2, 4}}
	if !sameIntGroups(got, want) {
		t.Fatalf("palindromePairs() = %v, want %v", got, want)
	}
}

func TestPalindromePairsWithPalindrome(t *testing.T) {
	got := palindromePairs([]string{"aba", ""})
	want := [][]int{{0, 1}, {1, 0}}
	if !sameIntGroups(got, want) {
		t.Fatalf("palindromePairs() = %v, want %v", got, want)
	}
}
