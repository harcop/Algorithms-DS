package leetcode

import "testing"

func TestFindCelebrityExampleOne(t *testing.T) {
	graph := NewCelebrity(3, 1)
	if got := graph.FindCelebrity(); got != 1 {
		t.Fatalf("FindCelebrity() = %d, want 1", got)
	}
}
