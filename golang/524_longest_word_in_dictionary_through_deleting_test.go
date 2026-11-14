package leetcode

import "testing"

func TestFindLongestWordExamples(t *testing.T) {
	if got := findLongestWord("abpcplea", []string{"ale", "apple", "monkey", "plea"}); got != "apple" {
		t.Fatalf("findLongestWord() = %q, want apple", got)
	}
	if got := findLongestWord("abpcplea", []string{"a", "b", "c"}); got != "a" {
		t.Fatalf("findLongestWord() = %q, want a", got)
	}
}
