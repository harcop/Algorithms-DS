package leetcode

import "testing"

func TestFindTheDifferenceExamples(t *testing.T) {
	if got := findTheDifference("abcd", "abcde"); got != 'e' {
		t.Fatalf("findTheDifference() = %q, want e", got)
	}
	if got := findTheDifference("", "y"); got != 'y' {
		t.Fatalf("findTheDifference() = %q, want y", got)
	}
}
