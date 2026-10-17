package leetcode

import "testing"

func TestLadderLengthExampleOne(t *testing.T) {
	got := ladderLength("hit", "cog", []string{"hot", "dot", "dog", "lot", "log", "cog"})
	if got != 5 {
		t.Fatalf("ladderLength() = %d, want 5", got)
	}
}

func TestLadderLengthExampleTwo(t *testing.T) {
	got := ladderLength("hit", "cog", []string{"hot", "dot", "dog", "lot", "log"})
	if got != 0 {
		t.Fatalf("ladderLength() = %d, want 0", got)
	}
}
