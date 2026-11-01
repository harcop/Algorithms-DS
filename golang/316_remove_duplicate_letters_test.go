package leetcode

import "testing"

func TestRemoveDuplicateLettersExample(t *testing.T) {
	if got := removeDuplicateLetters("bcabc"); got != "abc" {
		t.Fatalf("removeDuplicateLetters() = %q, want %q", got, "abc")
	}
	if got := removeDuplicateLetters("cbacdcbc"); got != "acdb" {
		t.Fatalf("removeDuplicateLetters() = %q, want %q", got, "acdb")
	}
}
