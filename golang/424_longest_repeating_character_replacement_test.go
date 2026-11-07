package leetcode

import "testing"

func TestCharacterReplacementExamples(t *testing.T) {
	if got := characterReplacement("ABAB", 2); got != 4 {
		t.Fatalf("characterReplacement(ABAB, 2) = %d, want 4", got)
	}
	if got := characterReplacement("AABABBA", 1); got != 4 {
		t.Fatalf("characterReplacement(AABABBA, 1) = %d, want 4", got)
	}
}
