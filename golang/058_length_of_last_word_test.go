package leetcode

import "testing"

func TestLengthOfLastWordExampleOne(t *testing.T) {
	got := lengthOfLastWord("Hello World")
	if got != 5 {
		t.Fatalf("lengthOfLastWord() = %d, want 5", got)
	}
}

func TestLengthOfLastWordExampleTwo(t *testing.T) {
	got := lengthOfLastWord("   fly me   to   the moon  ")
	if got != 4 {
		t.Fatalf("lengthOfLastWord() = %d, want 4", got)
	}
}
